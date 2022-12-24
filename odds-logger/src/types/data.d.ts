export type BasicDataType = {
  key: string
  id: number
  name: string
  note: string
}

export type SelectType = {
  label: string
  value: number
}

export interface DataType extends BasicDataType {
  index: number
  league_name: string
  result: string
}
