export type BasicDataType = {
  key: string
  index: number
  id: number
}

export type SelectType = {
  label: string
  value: number
}

export interface DataType extends BasicDataType {
  name: string
  note: string
}

export interface MatchInfoDataType extends BasicDataType {
  id: number
  home_team: string
  away_team: string
  game_time: string
  game_round: string
  game_result: string
  game_year: string
  league_name: string
  note: string
}

export interface MatchInfoTableType extends BasicDataType {
  index: number
  league_name: string
  vs: string
  result: string
  year: string
  round: string
  note: string
  time: string
}
