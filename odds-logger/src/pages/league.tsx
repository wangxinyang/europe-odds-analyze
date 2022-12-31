import { Alert, Button, Form, Input, message, Popconfirm, Space, Table, Tag } from 'antd'
import { useEffect, useState } from 'react'
import type { ColumnsType } from 'antd/es/table'
import { invoke } from '@tauri-apps/api'
import { error, success } from '../utils'

function League() {
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

  const columns: ColumnsType<DataType> = [
    {
      title: 'ID',
      dataIndex: 'index',
      key: 'index',
      render: (text) => <a>{text}</a>,
    },
    {
      title: '联赛名',
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

  const [form] = Form.useForm()
  const [data, setData] = useState<DataType[]>([])
  const [messageApi, contextHolder] = message.useMessage()

  // render bookmaker list data in page
  const render_list = (lists: DataType[]) => {
    // clear data
    setData([])
    lists.map((item, index) => {
      let data = { ...item, key: (index + 1).toString(), index: index + 1 }
      setData((prev) => [...prev, data])
    })
  }

  // initial list data
  useEffect(() => {
    const get_lists = async () => {
      let lists = await invoke<DataType[]>('get_league_lists')
      render_list(lists)
    }
    get_lists()
  }, [])

  // query bookmaker list
  const handleSearchInfo = async () => {
    let lists = await invoke<DataType[]>('get_league_lists')
    render_list(lists)
  }

  // handle bookmaker save
  const handleSaveInfo = async () => {
    try {
      const values = await form.validateFields()
      // call rust async function
      let lists = await invoke<DataType[]>('save_league_info', {
        name: values.name,
        note: values.note == undefined ? '' : values.note,
      })
      render_list(lists)
      success(messageApi, 'Successful: 保存成功')
    } catch (errorInfo) {
      error(messageApi, 'Failed: 保存失败, 请检查数据')
    }
  }

  const handleDelete = async (record: DataType) => {
    try {
      let { id } = record
      let lists = await invoke<DataType[]>('delete_league_info', { id })
      render_list(lists)
      success(messageApi, 'Successful: 删除成功')
    } catch (errorInfo) {
      error(messageApi, 'Failed: 删除失败, 请检查数据')
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
            <Button type="primary" onClick={handleSearchInfo}>
              查询
            </Button>
            <Button type="primary" danger onClick={handleSaveInfo}>
              保存
            </Button>
          </Space>
        </Form.Item>
      </Form>
      <Table columns={columns} dataSource={data} pagination={false} />
    </>
  )
}

export default League
