import { Button, Form, Input, message, Space } from 'antd'
import { useEffect } from 'react'
import type { ColumnsType } from 'antd/es/table'
import { invoke } from '@tauri-apps/api'
import { error, success } from '../utils'
import { useParams } from 'react-router-dom'

function LeagueUpdate() {
  const formItemLayout = {
    labelCol: { span: 4 },
    wrapperCol: { span: 8 },
  }

  const formTailLayout = {
    labelCol: { span: 4 },
    wrapperCol: { span: 8, offset: 4 },
  }

  interface DataType {
    key: string
    id: number
    index: number
    name: string
    note: string
  }

  const { id } = useParams<{ id: string }>()
  const [form] = Form.useForm()
  const [messageApi, contextHolder] = message.useMessage()

  // render the league data
  const render_league = (league: DataType) => {
    form.setFieldsValue({
      name: league.name,
      note: league.note,
    })
  }

  // initial list data
  useEffect(() => {
    const getLeagueById = async () => {
      let league = await invoke<DataType>('get_league_with_id', { id: parseInt(id as string) })
      render_league(league)
    }
    getLeagueById()
  }, [])

  // handle bookmaker update
  const handleSaveInfo = async () => {
    console.log('id is:', id)

    try {
      const values = await form.validateFields()
      // call rust async function
      await invoke('update_league_info', {
        id: parseInt(id as string),
        name: values.name,
        note: values.note == undefined ? '' : values.note,
      })
      // page back
      window.history.back()
      success(messageApi, 'Successful: 更新成功')
    } catch (errorInfo) {
      console.log(errorInfo)
      error(messageApi, 'Failed: 更新失败, 请检查数据')
    }
  }

  return (
    <>
      {contextHolder}
      <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 14 }} layout="horizontal">
        <Form.Item
          {...formItemLayout}
          name="name"
          label="联赛名称"
          rules={[
            {
              required: true,
              message: '请输入联赛名称',
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

export default LeagueUpdate
