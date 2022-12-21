import { invoke } from '@tauri-apps/api'
import { Button, Col, DatePicker, Form, Input, message, Popconfirm, Row, Select } from 'antd'
import Table, { ColumnsType } from 'antd/es/table'
import { useEffect, useState } from 'react'
import { error, success } from '../utils'

interface DataType {
  key: string
  id: number
  index: number
  legue_id: number
  name: string
  note: string
}

interface LeagueDataType {
  key: string
  id: number
  name: string
  note: string
}

interface TeamDataType {
  key: string
  id: number
  name: string
  note: string
}

const formItemLayout = {
  labelCol: { span: 4 },
  wrapperCol: { span: 16 },
}

const formTailLayout = {
  labelCol: { span: 4 },
  wrapperCol: { span: 8, offset: 4 },
}

function MatchList() {
  // form
  const [form] = Form.useForm()
  const [messageApi, contextHolder] = message.useMessage()
  const [data, setData] = useState<DataType[]>([])
  // selected league data
  const [selectedLeagueData, setSelectedLeaueData] = useState<number>(0)
  // team list data
  const [teamDataWithLeague, setTeamDataWithLeague] = useState<{ label: string; value: number }[]>(
    []
  )
  // league list data
  const [leagueData, setLeagueData] = useState<LeagueDataType[]>([])

  // init the team data with the league data
  useEffect(() => {
    let options: { label: string; value: number }[] = []
    const get_team_lists_with_league = async () => {
      try {
        let teams = await invoke<TeamDataType[]>('query_teams_with_league', {
          id: selectedLeagueData,
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
  }, [selectedLeagueData])

  const columns: ColumnsType<DataType> = [
    {
      title: 'ID',
      dataIndex: 'index',
      key: 'index',
      render: (text) => <a>{text}</a>,
    },
    {
      title: '联赛名',
      dataIndex: 'league_name',
      key: 'league_name',
    },
    {
      title: '球队名',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '备注',
      key: 'note',
      dataIndex: 'note',
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record, _index) => {
        return (
          <Popconfirm title="Sure to delete?" onConfirm={() => handleDelete(record)}>
            <a>Delete</a>
          </Popconfirm>
        )
      },
    },
  ]

  const selectLeagueDataOption = (data: LeagueDataType[]) => {
    let options: { label: string; value: number }[] = []
    data.map((item) => {
      options.push({
        value: item.id,
        label: item.name,
      })
    })

    return options
  }

  const handleDelete = async (record: DataType) => {
    try {
      let { id } = record
      let lists = await invoke<DataType[]>('delete_team_info', { id })
      //   render_list(lists)
      success(messageApi, 'Successful: 删除成功')
    } catch (errorInfo) {
      error(messageApi, 'Failed: 删除失败, 请检查数据')
    }
  }

  // change the select with league
  const handleLeagueChange = (id: number) => {
    setSelectedLeaueData(id)
    // clear second select content
    form.resetFields
    // console.log(form.resetFields)
  }

  const onSecondCityChange = () => {
    // setSecondTeam(value)
  }

  // query bookmaker list
  const handleSearchInfo = async () => {
    try {
      let lists = await invoke<DataType[]>('get_team_lists')
      render_list(lists)
    } catch (errorInfo) {
      error(messageApi, 'Failed: 查询失败, 请检查数据')
    }
  }

  // render team list data in page
  const render_list = (lists: DataType[]) => {
    // clear data
    setData([])
    lists.map((item, index) => {
      let data = { ...item, key: index.toString(), index: index + 1 }
      setData((prev) => [...prev, data])
    })
  }

  return (
    <>
      {contextHolder}
      <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 14 }} layout="horizontal">
        <Row>
          <Col span={12}>
            <Form.Item
              {...formItemLayout}
              name="league_id"
              label="联赛"
              rules={[
                {
                  required: true,
                  message: '请选择一个联赛',
                },
              ]}>
              <Select
                defaultValue={selectedLeagueData}
                value={selectedLeagueData}
                placeholder="选择一个联赛"
                onChange={handleLeagueChange}
                options={selectLeagueDataOption(leagueData)}
              />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item {...formItemLayout} name="game_year" label="赛季">
              <Input />
            </Form.Item>
          </Col>
        </Row>
        <Row>
          <Col span={12}>
            <Form.Item {...formItemLayout} name="game_round" label="轮次">
              <Input />
            </Form.Item>
          </Col>
        </Row>
        <Row gutter={1}>
          <Col span={12}>
            <Form.Item
              {...formItemLayout}
              name="home_team"
              label="主队"
              rules={[
                {
                  required: true,
                  message: '请选择球队名称',
                },
              ]}>
              <Select
                labelInValue={true}
                placeholder="选择球队"
                optionFilterProp="children"
                onChange={onSecondCityChange}
                options={teamDataWithLeague}
              />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item
              {...formItemLayout}
              name="away_team"
              label="客队"
              rules={[
                {
                  required: true,
                  message: '请选择球队名称',
                },
              ]}>
              <Select
                labelInValue={true}
                placeholder="选择球队"
                optionFilterProp="children"
                onChange={onSecondCityChange}
                options={teamDataWithLeague}
              />
            </Form.Item>
          </Col>
        </Row>
        <Row gutter={1}>
          <Col span={12}>
            <Form.Item {...formItemLayout} name="game_result" label="比赛结果">
              <Select
                placeholder="选择比赛结果"
                onChange={onSecondCityChange}
                options={[
                  { value: '3', label: '主胜' },
                  { value: '1', label: '平' },
                  { value: '0', label: '主负' },
                ]}
              />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item name="game_time" label="比赛时间">
              <DatePicker showTime format="YYYY-MM-DD HH:mm:ss" placeholder="选择比赛时间" />
            </Form.Item>
          </Col>
        </Row>
        <Row>
          <Col span={12}>
            <Form.Item {...formItemLayout} name="note" label="备注">
              <Input />
            </Form.Item>
          </Col>
        </Row>
        <Row>
          <Col span={12}>
            <Form.Item {...formTailLayout}>
              <Button type="primary" onClick={handleSearchInfo}>
                查询
              </Button>
            </Form.Item>
          </Col>
        </Row>
      </Form>
      <Table columns={columns} dataSource={data} />
    </>
  )
}

export default MatchList
