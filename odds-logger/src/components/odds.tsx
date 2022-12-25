import { invoke } from '@tauri-apps/api'
import { Col, Form, Input, Row, Select } from 'antd'
import { useEffect, useState } from 'react'
import { DataType, SelectType } from '../types/data'

interface IformItemLayout {
  labelCol: { span: number }
  wrapperCol: { span: number }
}

type FormItemLayoutProps = {
  formItemLayout: IformItemLayout
  index: number
  is_add: boolean
}

const formTailLayout = {
  labelCol: { span: 6 },
  wrapperCol: { span: 6, offset: 0 },
}

function Odds({ formItemLayout, index, is_add }: FormItemLayoutProps) {
  const [bookmakers, setBokkmakers] = useState<SelectType[]>([])

  useEffect(() => {
    const get_book_maker_list = async () => {
      let selectBookMakers: SelectType[] = []
      let bookMakers = await invoke<DataType[]>('get_book_maker_lists')
      bookMakers.map((item) => {
        selectBookMakers.push({ label: item.name, value: item.id })
      })
      setBokkmakers(selectBookMakers)
    }
    get_book_maker_list()
  }, [])

  return (
    <>
      <div
        style={{
          background: '#eee',
          position: 'relative',
          paddingTop: 20,
          marginBottom: 10,
          borderRadius: 15,
        }}>
        <Row>
          <Col span={12}>
            <Form.Item
              {...formItemLayout}
              name={'bookmaker_id' + index}
              label="赔率公司"
              rules={[
                {
                  required: is_add ? true : false,
                  message: '请选择赔率公司',
                },
              ]}>
              <Select
                placeholder="选择赔率公司"
                //     onChange={handleLeagueChange}
                options={bookmakers}
              />
            </Form.Item>
          </Col>
        </Row>
        <Row>
          <Col span={8}>
            <Form.Item {...formTailLayout} name={'home_win_start' + index} label="主胜">
              <Input />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item {...formTailLayout} name={'draw_start' + index} label="和">
              <Input />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item {...formTailLayout} name={'away_win_start' + index} label="主负">
              <Input />
            </Form.Item>
          </Col>
        </Row>
        <Row>
          <Col span={8}>
            <Form.Item {...formTailLayout} name={'home_win_end' + index} label="主胜(即)">
              <Input />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item {...formTailLayout} name={'draw_end' + index} label="和(即)">
              <Input />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item {...formTailLayout} name={'away_win_end' + index} label="主负(即)">
              <Input />
            </Form.Item>
          </Col>
        </Row>
        {/* <Row>
          <Col span={12}>
            <Form.Item {...formButtonLayout}>
              <Button type="primary">添加</Button>
              <Button type="primary" danger>
                删除
              </Button>
            </Form.Item>
          </Col>
        </Row> */}
      </div>
    </>
  )
}

export default Odds
