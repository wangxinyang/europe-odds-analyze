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

export interface OddsDataType extends BasicDataType {
  match_id: number
  bookmark_id: number
  home_win_start: number
  draw_start: number
  away_win_start: number
  home_win_end: number
  draw_end: number
  away_win_end: number
  note: number
}

export interface MatchInfoDataType extends BasicDataType {
  id: number
  league_id: number
  league_name: string
  home_team: string
  away_team: string
  game_time: string
  game_round: string
  game_result: string
  game_year: string
  history_note: string
  note: string
  oddsInfo: OddsDataType[]
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
