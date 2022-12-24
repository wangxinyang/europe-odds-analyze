import { Button, Col, DatePicker, Form, Input, Row, Select } from 'antd'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api'
import { error, success } from '../utils'
import { MessageInstance } from 'antd/es/message/interface'
import { DataType, MatchInfoDataType, SelectType } from '../types/data'
import Odds from './odds'
import * as moment from 'moment'

const formItemLayout = {
  labelCol: { span: 4 },
  wrapperCol: { span: 16 },
}

const formTailLayout = {
  labelCol: { span: 4 },
  wrapperCol: { span: 8, offset: 4 },
}

type MatchInfoProps = {
  is_add: boolean
  messageApi: MessageInstance
  handleValue?: Function
}

function MatchInfo({ is_add, messageApi, handleValue }: MatchInfoProps) {
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

  // query match infos
  const get_match_infos = async () => {
    const values = await form.validateFields()
    console.log(values)

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
    try {
      let matchInfos = await invoke<MatchInfoDataType[]>('query_match_info', { query })
      handleValue!(matchInfos)
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

  // handle bookmaker save
  const handleSaveInfo = async () => {
    try {
      const values = await form.validateFields()
      console.log('Received values of form: ', values)
      let game_time = moment(values.game_time).format('YYYY-MM-DD HH:mm:ss')
      console.log('game_time: ', game_time)
      console.log('game_time: ', moment(new Date()).format('YYYY-MM-DD HH:mm:ss'))
      let matchInfo = {
        league_id: values.leagueInfo.value,
        league_name: values.leagueInfo.label,
        home_team_id: values.home_team.value,
        away_team_id: values.away_team.value,
        home_team_name: values.home_team.label,
        away_team_name: values.away_team.label,
        game_time,
        game_year: values.game_year,
        game_round: values.game_round,
        game_result: values.game_result,
        note: values.note,
      }

      let oddsInfos = [
        {
          bookmaker_id: values.bookmaker_id1,
          home_win_start: values.home_win_start1,
          home_win_end: values.home_win_end1,
          draw_start: values.draw_start1,
          draw_end: values.draw_end1,
          away_win_start: values.away_win_start1,
          away_win_end: values.away_win_end1,
        },
        {
          bookmaker_id: values.bookmaker_id2,
          home_win_start: values.home_win_start2,
          home_win_end: values.home_win_end2,
          draw_start: values.draw_start2,
          draw_end: values.draw_end2,
          away_win_start: values.away_win_start2,
          away_win_end: values.away_win_end2,
        },
      ]

      // save match and odds
      await invoke<number>('save_match_odds', { matchInfo, oddsInfos })
      // clear second select content
      console.log(form)

      form.resetFields()
      success(messageApi, 'Successful: 保存成功')
    } catch (errorInfo) {
      console.log('Failed:', errorInfo)
      error(messageApi, 'Failed: 保存失败, 请检查数据')
    }
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
    setSelectedLeaueIndex(value)
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
          {is_add && (
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
          {!is_add && (
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
              label={is_add ? '主队' : '球队'}
              rules={[
                {
                  required: is_add ? true : false,
                  message: '请选择球队名称',
                },
              ]}>
              <Select labelInValue={true} placeholder="选择球队" options={teamDataWithLeague} />
            </Form.Item>
          </Col>

          {is_add && (
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
        <Row gutter={1}>
          {is_add && (
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
          {is_add && (
            <Col span={12}>
              <Form.Item name="game_time" label="比赛时间">
                <DatePicker showTime format="YYYY-MM-DD HH:mm:ss" placeholder="选择比赛时间" />
              </Form.Item>
            </Col>
          )}
        </Row>
        {is_add && (
          <Row>
            <Col span={12}>
              <Form.Item {...formItemLayout} name="note" label="备注">
                <Input />
              </Form.Item>
            </Col>
          </Row>
        )}
        {is_add && <Odds formItemLayout={formItemLayout} index={1} />}
        {is_add && <Odds formItemLayout={formItemLayout} index={2} />}
        <Row>
          <Col span={12}>
            <Form.Item {...formTailLayout}>
              {is_add ? (
                <Button type="primary" onClick={handleSaveInfo}>
                  保存
                </Button>
              ) : (
                <Button type="primary" onClick={get_match_infos}>
                  查询
                </Button>
              )}
              <Button type="primary" danger onClick={() => form.resetFields()}>
                清空
              </Button>
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </>
  )
}

export default MatchInfo
