import { Button, Form, Input, message, Space } from 'antd'
import { useEffect } from 'react'
import { invoke } from '@tauri-apps/api'
import { error, success } from '../utils'
import { useParams } from 'react-router-dom'
import { BookMakerDataType } from '../types/data'

function BookMakerUpdate() {
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
  const [messageApi, contextHolder] = message.useMessage()

  // render bookmaker list data in page
  const render_list = (bookmaker: BookMakerDataType) => {
    form.setFieldsValue({
      name: bookmaker.name,
      url: bookmaker.url,
      note: bookmaker.note,
    })
  }

  // initial list data
  useEffect(() => {
    const getBookMakerById = async () => {
      let bookMaker = await invoke<BookMakerDataType>('get_book_maker_with_id', {
        id: parseInt(id as string),
      })
      render_list(bookMaker)
    }
    getBookMakerById()
  }, [])

  // handle bookmaker update
  const handleSaveInfo = async () => {
    try {
      const values = await form.validateFields()
      // call rust async function
      await invoke('update_book_maker', {
        id: parseInt(id as string),
        name: values.name,
        url: values.url == undefined ? '' : values.url,
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

export default BookMakerUpdate
