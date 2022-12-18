import { Button, Form, Input, message, Popconfirm, Select, Table } from 'antd'
import { useEffect, useState } from 'react'
import type { ColumnsType } from 'antd/es/table'
import { invoke } from '@tauri-apps/api'
import { error, success } from '../utils'

function Team() {
  const formItemLayout = {
    labelCol: { span: 4 },
    wrapperCol: { span: 8 },
  }

  const formTailLayout = {
    labelCol: { span: 4 },
    wrapperCol: { span: 8, offset: 4 },
  }

  interface LeagueDataType {
    key: string
    id: number
    name: string
    note: string
  }

  interface DataType {
    key: string
    id: number
    index: number
    legue_id: number
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

  const [form] = Form.useForm()
  const [leagueData, setLeagueData] = useState<LeagueDataType[]>([])
  const [data, setData] = useState<DataType[]>([])
  const [messageApi, contextHolder] = message.useMessage()

  // render league list data in page
  const render_league_list = (lists: LeagueDataType[]) => {
    lists.map((item, index) => {
      let data = { ...item, key: index.toString() }
      setLeagueData((prev) => [...prev, data])
    })
  }

  // render team list data in page
  const render_list = (lists: DataType[]) => {
    // clear data
    setData([])
    lists.map((item, index) => {
      console.log(item)

      let data = { ...item, key: index.toString(), index: index + 1 }
      setData((prev) => [...prev, data])
    })
  }

  // initial league list data
  useEffect(() => {
    const get_league_lists = async () => {
      let lists = await invoke<DataType[]>('get_league_lists')
      render_league_list(lists)
    }
    get_league_lists()
  }, [])

  // initial team list data
  useEffect(() => {
    const get_team_lists = async () => {
      let lists = await invoke<DataType[]>('get_team_lists')
      render_list(lists)
    }
    get_team_lists()
  }, [])

  // query bookmaker list
  const handleSearchInfo = async () => {
    try {
      let lists = await invoke<DataType[]>('get_team_lists')
      render_list(lists)
    } catch (errorInfo) {
      error(messageApi, 'Failed: 查询失败, 请检查数据')
    }
  }

  // handle bookmaker save
  const handleSaveInfo = async () => {
    try {
      const values = await form.validateFields()
      // call rust async function
      let lists = await invoke<DataType[]>('save_team_info', {
        id: parseInt(values.league_id, 10),
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
      let lists = await invoke<DataType[]>('delete_team_info', { id })
      render_list(lists)
      success(messageApi, 'Successful: 删除成功')
    } catch (errorInfo) {
      error(messageApi, 'Failed: 删除失败, 请检查数据')
    }
  }

  const onChange = (value: string) => {
    console.log(`selected ${value}`)
  }

  const onSearch = (value: string) => {
    console.log('search:', value)
  }

  const options = (data: LeagueDataType[]) => {
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
          <Select
            showSearch
            placeholder="Select a league"
            optionFilterProp="children"
            onChange={onChange}
            onSearch={onSearch}
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
            options={options(leagueData)}
          />
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
          <Button type="primary" onClick={handleSearchInfo}>
            查询
          </Button>
          <Button type="primary" onClick={handleSaveInfo}>
            保存
          </Button>
        </Form.Item>
      </Form>
      <Table columns={columns} dataSource={data} />
    </>
  )
}

export default Team
