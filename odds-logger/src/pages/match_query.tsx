import { useState } from 'react'
import { invoke } from '@tauri-apps/api'
import { message, Popconfirm, Space } from 'antd'
import Table, { ColumnsType } from 'antd/es/table'
import { DataType, MatchInfoDataType, MatchInfoTableType } from '../types/data'
import { error, success } from '../utils'
import MatchInfo from '../components/match_info'
import { Link } from 'react-router-dom'
import dayjs from 'dayjs'

function MatchQuery() {
  const [messageApi, contextHolder] = message.useMessage()
  const [tableData, setTableData] = useState<MatchInfoTableType[]>([])

  const columns: ColumnsType<MatchInfoTableType> = [
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
      title: '赛季',
      dataIndex: 'year',
      key: 'year',
    },
    {
      title: '轮次',
      dataIndex: 'round',
      key: 'round',
    },
    {
      title: '比赛对阵',
      dataIndex: 'vs',
      key: 'vs',
    },
    {
      title: '预测结果',
      dataIndex: 'predict_result',
      key: 'predict_result',
      render: (result) => {
        if (result === '3') {
          return '主胜'
        } else if (result === '1') {
          return '平局'
        } else if (result === '0') {
          return '主负'
        } else {
          return ''
        }
      },
    },
    {
      title: '实际结果',
      dataIndex: 'result',
      key: 'result',
      render: (result) => {
        if (result === '3') {
          return '主胜'
        } else if (result === '1') {
          return '平局'
        } else if (result === '0') {
          return '主负'
        } else {
          return ''
        }
      },
    },
    {
      title: '比赛时间',
      dataIndex: 'time',
      key: 'time',
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record, _index) => {
        return (
          <Space size={8}>
            <Link to={`/matchDetail/${record.id}`}>详情</Link>
            <Popconfirm title="Sure to delete?" onConfirm={() => handleDelete(record)}>
              <a>删除</a>
            </Popconfirm>
          </Space>
        )
      },
    },
  ]

  const handleDelete = async (record: MatchInfoTableType) => {
    try {
      let { id } = record
      render_list(id)
      await invoke<DataType[]>('delete_match_info', { id })
      success(messageApi, 'Successful: 删除成功')
    } catch (errorInfo) {
      error(messageApi, 'Failed: 删除失败, 请检查数据')
    }
  }

  // render team list data in page
  const render_list = (id: number) => {
    let new_data = tableData.filter((item) => item.id !== id)
    setTableData(new_data)
  }

  // init the table info list data
  const getMatchInfoTableData = (data: MatchInfoDataType[]) => {
    let result: MatchInfoTableType[] = []
    data.map((item, index) => {
      result.push({
        key: item.id.toString(),
        id: item.id,
        index: index + 1,
        league_name: item.league_name,
        vs: item.home_team + ' vs ' + item.away_team,
        year: item.game_year,
        round: item.game_round,
        result: item.game_result,
        predict_result: item.predict_game_result,
        time: dayjs(item.game_time).format('YYYY/MM/DD HH:mm:ss'),
        note: item.note,
      })
    })
    setTableData(result)
  }

  const getRowClassName = (record: MatchInfoTableType, index: number) => {
    console.log('record is', record)

    let className = ''
    if (record.result !== '') {
      if (record.result !== record.predict_result) {
        className = 'error_predict'
      } else {
        className = 'correct_predict'
      }
    }
    return className
  }

  return (
    <>
      {contextHolder}
      <MatchInfo
        is_add={false}
        is_update={false}
        messageApi={messageApi}
        handleValue={getMatchInfoTableData}
      />
      <Table columns={columns} dataSource={tableData} rowClassName={getRowClassName} />
    </>
  )
}

export default MatchQuery
