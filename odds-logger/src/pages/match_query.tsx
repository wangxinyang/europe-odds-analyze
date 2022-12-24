import { useState } from 'react'
import { invoke } from '@tauri-apps/api'
import { message, Popconfirm } from 'antd'
import Table, { ColumnsType } from 'antd/es/table'
import MatchInfo from '../components/match_info'
import { DataType } from '../types/data'
import { error, success } from '../utils'

function MatchQuery() {
  const [messageApi, contextHolder] = message.useMessage()
  const [tableData, setTableData] = useState<DataType[]>([])

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
      title: '比赛对阵',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '结果',
      dataIndex: 'result',
      key: 'result',
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
          <>
            <Popconfirm title="Sure to delete?" onConfirm={() => handleDelete(record)}>
              <a>详情</a>
            </Popconfirm>
            <Popconfirm title="Sure to delete?" onConfirm={() => handleDelete(record)}>
              <a>删除</a>
            </Popconfirm>
          </>
        )
      },
    },
  ]

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

  // render team list data in page
  const render_list = (lists: DataType[]) => {
    // clear data
    // setData([])
    lists.map((item, index) => {
      let data = { ...item, key: index.toString(), index: index + 1 }
      // setData((prev) => [...prev, data])
    })
  }

  const getMatchInfoTableData = (data: DataType[]) => {
    console.log('data is:', data)
    let result: DataType[] = []
    data.map((item, index) => {
      result.push({
        key: '1',
        id: 1,
        name: 'ghaha',
        index: 1,
        league_name: 'test1',
        result: 'sl',
        note: 'hello',
      })
    })
    setTableData(result)
  }

  return (
    <>
      {contextHolder}
      <MatchInfo is_add={false} messageApi={messageApi} handleValue={getMatchInfoTableData} />
      <Table columns={columns} dataSource={tableData} />
    </>
  )
}

export default MatchQuery
