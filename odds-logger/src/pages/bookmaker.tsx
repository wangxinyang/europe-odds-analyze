import { Alert, Button, Form, Input, message, Space, Table, Tag } from 'antd'
import { useEffect, useState } from 'react'
import type { ColumnsType } from 'antd/es/table'
import { invoke } from '@tauri-apps/api'
import { error } from '../utils'

function BookMaker() {
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
    url: string
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
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: 'Url',
      dataIndex: 'url',
      key: 'url',
    },
    {
      title: 'Note',
      key: 'note',
      dataIndex: 'note',
    },
    {
      title: 'Action',
      key: 'action',
      render: (_, record, _index) => {
        return (
          <Space size="middle">
            <a onClick={() => handleDelete(record)}>Delete</a>
          </Space>
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
      let lists = await invoke<DataType[]>('get_book_maker_lists')
      render_list(lists)
    }
    get_lists()
  }, [])

  // query bookmaker list
  const handleSearchInfo = async () => {
    let lists = await invoke<DataType[]>('get_book_maker_lists')
    render_list(lists)
  }

  // handle bookmaker save
  const handleSaveInfo = async () => {
    try {
      const values = await form.validateFields()

      // call rust async function
      let lists = await invoke<DataType[]>('save_book_maker_info', {
        name: values.name,
        url: values.url,
        note: values.note,
      })
      render_list(lists)
    } catch (errorInfo) {
      error(messageApi, 'Failed: 保存公司数据失败, 请检查数据', 3)
    }
  }

  const handleDelete = async (record: DataType) => {
    let { id } = record
    let lists = await invoke<DataType[]>('delete_book_maker_info', { id })
    render_list(lists)
  }

  return (
    <>
      {contextHolder}
      <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 14 }} layout="horizontal">
        <Form.Item
          {...formItemLayout}
          name="name"
          label="公司名"
          rules={[
            {
              required: true,
              message: '请输入公司名',
            },
          ]}>
          <Input />
        </Form.Item>
        <Form.Item {...formItemLayout} name="url" label="官网">
          <Input />
        </Form.Item>
        <Form.Item {...formItemLayout} name="note" label="备注">
          <Input />
        </Form.Item>
        <Form.Item {...formTailLayout}>
          <Button type="primary" onClick={handleSearchInfo}>
            查询
          </Button>
          <Button type="primary" onClick={handleSaveInfo}>
            保存
          </Button>
        </Form.Item>
      </Form>
      <Table columns={columns} dataSource={data} pagination={false} />
    </>
  )
}

export default BookMaker
