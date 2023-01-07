import { Button, Form, Input, message, Popconfirm, Space, Table } from 'antd'
import { useEffect, useState } from 'react'
import type { ColumnsType } from 'antd/es/table'
import { invoke } from '@tauri-apps/api'
import { error, success } from '../utils'
import { useParams } from 'react-router-dom'

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
  // const render_list = (lists: DataType[]) => {
  //   // clear data
  //   setData([])
  //   lists.map((item, index) => {
  //     let data = { ...item, key: (index + 1).toString(), index: index + 1 }
  //     setData((prev) => [...prev, data])
  //   })
  // }

  // initial list data
  // useEffect(() => {
  //   const get_lists = async () => {
  //     let lists = await invoke<DataType[]>('get_book_maker_lists')
  //     render_list(lists)
  //   }
  //   get_lists()
  // }, [])

  // handle bookmaker save
  const handleSaveInfo = async () => {
    // try {
    //   const values = await form.validateFields()
    //   // call rust async function
    //   let lists = await invoke<DataType[]>('save_book_maker_info', {
    //     name: values.name,
    //     url: values.url == undefined ? '' : values.url,
    //     note: values.note == undefined ? '' : values.note,
    //   })
    //   render_list(lists)
    //   success(messageApi, 'Successful: 保存成功')
    // } catch (errorInfo) {
    //   error(messageApi, 'Failed: 保存失败, 请检查数据')
    // }
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
