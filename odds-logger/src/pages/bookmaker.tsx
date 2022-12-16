import { Button, Form, Input, Space, Table, Tag } from 'antd'
import { useEffect } from 'react'
import type { ColumnsType } from 'antd/es/table'

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
    render: (_, record) => (
      <Space size="middle">
        <a>Delete</a>
      </Space>
    ),
  },
]

const data: DataType[] = [
  {
    key: '1',
    index: 1,
    name: '威廉希尔',
    url: 'www.baidu.com',
    note: '威廉希尔',
  },
  {
    key: '2',
    index: 2,
    name: '威廉希尔',
    url: 'www.baidu.com',
    note: '威廉希尔',
  },
  {
    key: '3',
    index: 3,
    name: '威廉希尔',
    url: 'www.baidu.com',
    note: '威廉希尔',
  },
]

function BookMaker() {
  const [form] = Form.useForm()

  useEffect(() => {
    form.validateFields(['nickname'])
  }, [form])

  // handle bookmaker save
  const handleSaveInfo = async () => {
    try {
      const values = await form.validateFields()
      // call rust async function

      console.log('Success:', values)
    } catch (errorInfo) {
      console.log('Failed:', errorInfo)
    }
  }

  return (
    <>
      <Form form={form} labelCol={{ span: 4 }} wrapperCol={{ span: 14 }} layout="horizontal">
        <Form.Item
          {...formItemLayout}
          name="username"
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
