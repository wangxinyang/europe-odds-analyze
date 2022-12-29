import { invoke } from '@tauri-apps/api'
import { Col, Form, FormInstance, Input, Row, Select } from 'antd'
import { MinusCircleOutlined } from '@ant-design/icons'
import { useEffect, useState } from 'react'
import { DataType, OddsDataType, SelectType } from '../types/data'

const formItemLayout = {
  labelCol: { span: 4 },
  wrapperCol: { span: 16 },
}

type FormItemLayoutProps = {
  is_add: boolean
  initValue?: OddsDataType
  form?: FormInstance
  listKey: number
  name: number
  remove: Function
}

const formTailLayout = {
  labelCol: { span: 6 },
  wrapperCol: { span: 6, offset: 0 },
}

function Odds({ is_add, form, listKey, name, remove }: FormItemLayoutProps) {
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
    <div
      key={listKey}
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
            name={is_add ? [name, 'bookmaker'] : [name, 'bookmaker_name']}
            label="赔率公司"
            rules={[
              {
                required: is_add ? true : false,
                message: '请选择赔率公司',
              },
            ]}>
            {is_add ? (
              <Select
                labelInValue={true}
                placeholder="选择赔率公司"
                //     onChange={handleLeagueChange}
                options={bookmakers}
              />
            ) : (
              <Input disabled />
            )}
          </Form.Item>
        </Col>
      </Row>
      <Row>
        <Col span={8}>
          <Form.Item {...formTailLayout} name={[name, 'home_win_start']} label="主胜">
            <Input />
          </Form.Item>
        </Col>
        <Col span={8}>
          <Form.Item {...formTailLayout} name={[name, 'draw_start']} label="和">
            <Input />
          </Form.Item>
        </Col>
        <Col span={8}>
          <Form.Item {...formTailLayout} name={[name, 'away_win_start']} label="主负">
            <Input />
          </Form.Item>
        </Col>
      </Row>
      <Row>
        <Col span={8}>
          <Form.Item {...formTailLayout} name={[name, 'home_win_end']} label="主胜(即)">
            <Input />
          </Form.Item>
        </Col>
        <Col span={8}>
          <Form.Item {...formTailLayout} name={[name, 'draw_end']} label="和(即)">
            <Input />
          </Form.Item>
        </Col>
        <Col span={8}>
          <Form.Item {...formTailLayout} name={[name, 'away_win_end']} label="主负(即)">
            <Input />
          </Form.Item>
        </Col>
      </Row>
      <MinusCircleOutlined onClick={() => remove(name)} />
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
  )
}

export default Odds
