import { Button, Col, DatePicker, Form, Input, Row, Select, Space } from 'antd'
import { PlusOutlined } from '@ant-design/icons'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api'
import {
  DataType,
  MatchInfoDataType,
  MatchInfoFormType,
  OddsDataType,
  OddsFormType,
  SelectType,
} from '../types/data'
import TextArea from 'antd/es/input/TextArea'
import { MessageInstance } from 'antd/es/message/interface'
import dayjs from 'dayjs'
import { error, success } from '../utils'
import Odds from './odds'

const formItemLayout = {
  labelCol: { span: 4 },
  wrapperCol: { span: 16 },
}

const formTailLayout = {
  labelCol: { span: 4 },
  wrapperCol: { span: 8, offset: 4 },
}

type MatchInfoProps = {
  match_id?: string
  is_add: boolean
  is_update: boolean
  messageApi: MessageInstance
  handleValue?: Function
}

function MatchInfo({ match_id, is_add, is_update, messageApi, handleValue }: MatchInfoProps) {
  // form
  const [form] = Form.useForm()
  // league list data
  const [leagueData, setLeagueData] = useState<DataType[]>([])
  // selected default league data
  const [selectedLeagueIndex, setSelectedLeaueIndex] = useState<number>(0)
  // team list data
  const [teamDataWithLeague, setTeamDataWithLeague] = useState<SelectType[]>([])
  // bookmaker data info
  const [bookmakers, setBokkmakers] = useState<SelectType[]>([])
  // update data
  const [updateData, setUpdateData] = useState<MatchInfoDataType>({} as MatchInfoDataType)

  // initial league list data
  useEffect(() => {
    const get_league_lists = async () => {
      let lists = await invoke<DataType[]>('get_league_lists')
      render_league_list(lists)
    }
    get_league_lists()
  }, [])

  // init the team data with the league data
  useEffect(() => {
    let options: { label: string; value: number }[] = []
    const get_team_lists_with_league = async () => {
      try {
        let teams = await invoke<DataType[]>('query_teams_with_league', {
          id: selectedLeagueIndex,
        })

        teams.map((item) => {
          options.push({
            value: item.id,
            label: item.name,
          })
        })
      } catch (errorInfo) {
        console.log(errorInfo)
        error(messageApi, 'Failed: 初始化球队数据失败, 请检查数据')
      }

      setTeamDataWithLeague(options)
    }
    get_team_lists_with_league()
  }, [selectedLeagueIndex])

  useEffect(() => {
    const get_book_maker_list = async () => {
      let selectBookMakers: SelectType[] = []
      let bookMakers = await invoke<DataType[]>('get_book_maker_lists')
      bookMakers.map((item) => {
        selectBookMakers.push({ label: item.name, value: item.id })
      })
      setBokkmakers(selectBookMakers)
    }
    get_book_maker_list()
  }, [])

  // query match info data with query mode
  useEffect(() => {
    if (!is_add) {
      get_match_infos()
    }
  }, [])

  useEffect(() => {
    form.setFieldsValue({
      leagueInfo: updateData.league_name,
      history_note: updateData.history_note,
      game_year: updateData.game_year,
      game_round: updateData.game_round,
      home_team: updateData.home_team,
      away_team: updateData.away_team,
      game_result: updateData.game_result,
      predict_game_result: updateData.predict_game_result,
      game_time: updateData.game_time ? dayjs(updateData.game_time, 'YYYY/MM/DD HH:mm:ss') : '',
      note: updateData.note,
    })
  }, [updateData])

  // query match infos
  const get_match_infos = async () => {
    try {
      const values = await form.validateFields()
      let query = {
        book_maker_id: values.bookmaker_id ? values.bookmaker_id : 0,
        league_id: values.leagueInfo ? values.leagueInfo.value : 0,
        team_id: values.home_team ? values.home_team : 0,
        game_year: values.game_year,
        game_round: values.game_round,
        is_desc: true,
        cursor: 100,
        page_size: 10,
      }
      let matchInfos = await invoke<MatchInfoDataType[]>('query_match_info', { query })
      // set table data with query mode
      if (!is_add && handleValue) {
        handleValue(matchInfos)
      } else if (is_update && match_id) {
        // update mode
        let matchInfo = matchInfos.find((item) => item.id === parseInt(match_id))
        // query odds info by match id
        if (matchInfo) {
          let odds = await invoke<OddsDataType[]>('query_odds_by_id', { id: matchInfo.id })
          matchInfo.oddsInfo = odds
          setUpdateData(matchInfo)
        }
      }
    } catch (err) {
      console.log('err is', err)
      error(messageApi, 'Failed: 查询失败, 请检查数据')
    }
  }

  // render league list data in page
  const render_league_list = (lists: DataType[]) => {
    lists.map((item, index) => {
      let data = { ...item, key: index.toString() }
      if (data) {
        setLeagueData((prev) => [...prev, data])
      }
    })
  }

  // handle bookmaker update
  const handleUpdateInfo = async () => {
    try {
      const values = await form.validateFields()
      console.log(values)

      // if odds data was not input
      if (values.odds === undefined) {
        error(messageApi, 'Failed: 保存失败, 请输入赔率数据')
        return
      }
      const matchInfo = buildMatchInfoByUpdate(values)
      const oddsInfos = buildOddsInfoByUpdate(values.odds)
      await invoke<number>('update_match_odds', { matchInfo, oddsInfos })
      success(messageApi, 'Successful: 更新成功')
    } catch (err) {
      console.log('handleUpdateInfo error is:', err)
      error(messageApi, 'Failed: 更新失败, 请检查数据')
    }
  }

  // handle bookmaker save
  const handleSaveInfo = async () => {
    try {
      const values = await form.validateFields()
      // if odds data was not input
      if (values.odds === undefined) {
        error(messageApi, 'Failed: 保存失败, 请输入赔率数据')
        return
      }
      const matchInfo = buildMatchInfoBySave(values)
      const oddsInfos = buildOddsInfoBySave(values.odds)
      // save match and odds
      await invoke<number>('save_match_odds', { matchInfo, oddsInfos })
      // clear second select content
      form.resetFields()
      success(messageApi, 'Successful: 保存成功')
    } catch (errorInfo) {
      console.log('Failed:', errorInfo)
      error(messageApi, 'Failed: 保存失败, 请检查数据')
    }
  }

  const buildMatchInfoBySave = (values: MatchInfoFormType) => {
    let matchInfo = {
      id: 0,
      league_id: values.leagueInfo.value,
      league_name: values.leagueInfo.label,
      home_team_id: values.home_team.value,
      away_team_id: values.away_team.value,
      home_team_name: values.home_team.label,
      away_team_name: values.away_team.label,
      game_time: values.game_time
        ? dayjs(values.game_time).format('YYYY-MM-DD HH:mm:ss')
        : undefined,
      game_year: values.game_year,
      game_round: values.game_round,
      game_result: values.game_result,
      predict_game_result: values.predict_game_result,
      history_note: values.history_note,
      note: values.note,
    }
    return matchInfo
  }

  const buildMatchInfoByUpdate = (values: MatchInfoFormType) => {
    let matchInfo = {
      id: parseInt(match_id as string),
      league_id: updateData.league_id,
      league_name: values.leagueInfo,
      home_team_id: updateData.home_team_id,
      away_team_id: updateData.away_team_id,
      home_team_name: values.home_team,
      away_team_name: values.away_team,
      game_time: values.game_time
        ? dayjs(values.game_time).format('YYYY-MM-DD HH:mm:ss')
        : undefined,
      game_year: values.game_year,
      game_round: values.game_round,
      game_result: values.game_result,
      predict_game_result: values.predict_game_result,
      history_note: values.history_note,
      note: values.note,
    }
    return matchInfo
  }

  const buildOddsInfoBySave = (odds: OddsFormType[]) => {
    let oddsInfos: OddsDataType[] = []
    odds.map((item) => {
      oddsInfos.push({
        ...item,
        id: 0,
        match_id: 0,
        bookmaker_id: item.bookmaker.value as number,
        bookmaker_name: item.bookmaker.label,
      })
    })
    return oddsInfos
  }

  const buildOddsInfoByUpdate = (odds: OddsFormType[]) => {
    let oddsInfos: OddsDataType[] = []
    odds.map((item, index) => {
      oddsInfos.push({
        ...item,
        id: item.id ? updateData.oddsInfo[index].id : 0,
        match_id: parseInt(match_id as string),
        bookmaker_id: (item.bookmaker_name as SelectType).value!
          ? (item.bookmaker_name as SelectType).value!
          : item.bookmaker_id,
        bookmaker_name: (item.bookmaker_name as SelectType).label
          ? (item.bookmaker_name as SelectType).label
          : (item.bookmaker_name as string),
      })
    })
    return oddsInfos
  }

  // build league select data info
  const selectLeagueDataOption = (data: DataType[]) => {
    let options: SelectType[] = []
    data.map((item) => {
      options.push({
        value: item.id,
        label: item.name,
      })
    })
    return options
  }

  // change the select with league
  const handleLeagueChange = (league: SelectType) => {
    let { value } = league
    setSelectedLeaueIndex(value as number)
  }

  return (
    <>
      <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 14 }} layout="horizontal">
        <Row>
          <Col span={12}>
            <Form.Item
              {...formItemLayout}
              name="leagueInfo"
              label="联赛"
              rules={[
                {
                  required: is_add ? true : false,
                  message: '请选择一个联赛',
                },
              ]}>
              <Select
                labelInValue={true}
                placeholder="选择联赛"
                onChange={handleLeagueChange}
                options={selectLeagueDataOption(leagueData)}
              />
            </Form.Item>
          </Col>
          {(is_add || is_update) && (
            <Col span={12}>
              <Form.Item {...formItemLayout} name="history_note" label="往绩">
                <Input />
              </Form.Item>
            </Col>
          )}
        </Row>
        <Row>
          <Col span={12}>
            <Form.Item {...formItemLayout} name="game_year" label="赛季">
              {/* <Input key={updateData.game_year} defaultValue={updateData.game_year} /> */}
              <Input />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item {...formItemLayout} name="game_round" label="轮次">
              <Input />
            </Form.Item>
          </Col>
        </Row>
        <Row gutter={1}>
          {!is_add && !is_update && (
            <Col span={12}>
              <Form.Item {...formItemLayout} name={'bookmaker_id'} label="赔率公司">
                <Select
                  placeholder="选择赔率公司"
                  //     onChange={handleLeagueChange}
                  options={bookmakers}
                />
              </Form.Item>
            </Col>
          )}
          <Col span={12}>
            <Form.Item
              {...formItemLayout}
              name="home_team"
              label={is_add || is_update ? '主队' : '球队'}
              rules={[
                {
                  required: is_add ? true : false,
                  message: '请选择球队名称',
                },
              ]}>
              <Select labelInValue={true} placeholder="选择球队" options={teamDataWithLeague} />
            </Form.Item>
          </Col>
          {(is_add || is_update) && (
            <Col span={12}>
              <Form.Item {...formItemLayout} name="predict_game_result" label="预测结果">
                <Select
                  placeholder="预测比赛结果"
                  options={[
                    { value: '3', label: '主胜' },
                    { value: '1', label: '平' },
                    { value: '0', label: '主负' },
                  ]}
                />
              </Form.Item>
            </Col>
          )}
        </Row>
        <Row gutter={1}>
          {(is_add || is_update) && (
            <Col span={12}>
              <Form.Item
                {...formItemLayout}
                name="away_team"
                label="客队"
                rules={[
                  {
                    required: is_add ? true : false,
                    message: '请选择球队名称',
                  },
                ]}>
                <Select labelInValue={true} placeholder="选择球队" options={teamDataWithLeague} />
              </Form.Item>
            </Col>
          )}
          {(is_add || is_update) && (
            <Col span={12}>
              <Form.Item {...formItemLayout} name="game_result" label="比赛结果">
                <Select
                  placeholder="选择比赛结果"
                  options={[
                    { value: '3', label: '主胜' },
                    { value: '1', label: '平' },
                    { value: '0', label: '主负' },
                  ]}
                />
              </Form.Item>
            </Col>
          )}
        </Row>
        {(is_add || is_update) && (
          <Row>
            <Col span={12}>
              <Form.Item {...formItemLayout} name="note" label="备注">
                <TextArea rows={3} placeholder="赔率记忆备注" />
              </Form.Item>
            </Col>
            {(is_add || is_update) && (
              <Col span={12}>
                <Form.Item name="game_time" label="比赛时间">
                  <DatePicker showTime format="YYYY-MM-DD HH:mm:ss" placeholder="选择比赛时间" />
                </Form.Item>
              </Col>
            )}
          </Row>
        )}
        {is_add && (
          <Form.List name="odds">
            {(fields, { add, remove }) => {
              return (
                <>
                  {fields.map(({ key, name, ...restField }) => (
                    <Odds key={key} is_add={is_add} listKey={key} name={name} remove={remove} />
                  ))}
                  <Button
                    style={{ width: '100%', marginTop: 8, marginBottom: 18 }}
                    type="dashed"
                    onClick={() => add()}
                    icon={<PlusOutlined />}>
                    添加赔率数据
                  </Button>
                </>
              )
            }}
          </Form.List>
        )}
        {is_update && updateData.oddsInfo && (
          <Form.List name="odds" initialValue={updateData.oddsInfo}>
            {(fields, { add, remove }) => {
              return (
                <>
                  {fields.map(({ key, name, ...restField }) => {
                    return (
                      <Odds key={key} is_add={false} listKey={key} name={name} remove={remove} />
                    )
                  })}
                  <Button
                    style={{ width: '100%', marginTop: 8, marginBottom: 18 }}
                    type="dashed"
                    onClick={() => add()}
                    icon={<PlusOutlined />}>
                    添加赔率数据
                  </Button>
                </>
              )
            }}
          </Form.List>
        )}
        <Row>
          <Col span={12}>
            <Form.Item {...formTailLayout}>
              <Space size={8}>
                {is_add && (
                  <>
                    <Button type="primary" danger onClick={handleSaveInfo}>
                      保存
                    </Button>
                    <Button type="primary" onClick={() => form.resetFields()}>
                      清空
                    </Button>
                  </>
                )}
                {is_update && (
                  <Button type="primary" danger onClick={handleUpdateInfo}>
                    更新
                  </Button>
                )}
                {!is_add && !is_update && (
                  <Button type="primary" onClick={get_match_infos}>
                    查询
                  </Button>
                )}
                {(is_add || is_update) && (
                  <Button type="primary" onClick={() => window.history.back()}>
                    返回
                  </Button>
                )}
              </Space>
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </>
  )
}

export default MatchInfo
