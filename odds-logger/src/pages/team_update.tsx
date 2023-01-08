import { Button, Form, Input, message, Select, Space } from 'antd'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api'
import { error, success } from '../utils'
import { useParams } from 'react-router-dom'
import { DataType, TeamDataType } from '../types/data'

function TeamUpdate() {
  const formItemLayout = {
    labelCol: { span: 4 },
    wrapperCol: { span: 8 },
  }

  const formTailLayout = {
    labelCol: { span: 4 },
    wrapperCol: { span: 8, offset: 4 },
  }

  const { id } = useParams<{ id: string }>()
  const [form] = Form.useForm()
  const [leagueData, setLeagueData] = useState<DataType[]>([])
  const [messageApi, contextHolder] = message.useMessage()

  // render league list data in page
  const renderLeagueList = (lists: DataType[]) => {
    lists.map((item, index) => {
      let data = { ...item, key: index.toString() }
      setLeagueData((prev) => [...prev, data])
    })
  }

  // render team data in page
  const renderTeamInfo = (team: TeamDataType) => {
    form.setFieldsValue({
      league_id: team.league_id,
      name: team.name,
      note: team.note,
    })
  }

  // initial league list data
  useEffect(() => {
    const getLeagueLists = async () => {
      let lists = await invoke<DataType[]>('get_league_lists')
      renderLeagueList(lists)
    }
    getLeagueLists()
  }, [])

  // initial team data
  useEffect(() => {
    const getTeam = async () => {
      let team = await invoke<TeamDataType>('get_team_with_id', { id: parseInt(id as string) })
      renderTeamInfo(team)
    }
    getTeam()
  }, [])

  // handle team update
  const handleSaveInfo = async () => {
    try {
      const values = await form.validateFields()
      // call rust async function
      await invoke('update_team_info', {
        id: parseInt(id as string),
        lid: parseInt(values.league_id),
        name: values.name,
        note: values.note == undefined ? '' : values.note,
      })
      success(messageApi, 'Successful: 更新成功')
    } catch (errorInfo) {
      console.log(errorInfo)
      error(messageApi, 'Failed: 更新失败, 请检查数据')
    }
  }

  const options = (data: DataType[]) => {
    let options: { label: string; value: number }[] = []
    data.map((item) => {
      options.push({
        value: item.id,
        label: item.name,
      })
    })
    return options
  }

  return (
    <>
      {contextHolder}
      <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 14 }} layout="horizontal">
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
          <Select placeholder="选择联赛" options={options(leagueData)} />
        </Form.Item>
        <Form.Item
          {...formItemLayout}
          name="name"
          label="球队名称"
          rules={[
            {
              required: true,
              message: '请输入球队名称',
            },
          ]}>
          <Input />
        </Form.Item>
        <Form.Item {...formItemLayout} name="note" label="备注">
          <Input />
        </Form.Item>
        <Form.Item {...formTailLayout}>
          <Space size={8}>
            <Button type="primary" danger onClick={handleSaveInfo}>
              更新
            </Button>
            <Button type="primary" onClick={() => window.history.back()}>
              返回
            </Button>
          </Space>
        </Form.Item>
      </Form>
    </>
  )
}

export default TeamUpdate
