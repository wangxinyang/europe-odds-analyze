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
  id: number
  match_id: number
  bookmaker_id: number
  bookmaker_name: string
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
  home_team_id: number
  away_team_id: number
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

export interface MatchOddsFormType {
  leagueInfo: SelectType
  home_team: SelectType
  away_team: SelectType
  game_time: MomentInput
  game_year: String
  game_round: String
  game_result: String
  history_note: String
  note: String
  match_id: number
  // TODO: not confirmed
  bookmaker0: SelectType
  home_win_start0: String
  home_win_end0: String
  draw_start0: String
  draw_end0: String
  away_win_start0: String
  away_win_end0: String
  bookmaker1: SelectType
  home_win_start1: String
  home_win_end1: String
  draw_start1: String
  draw_end1: String
  away_win_start1: String
  away_win_end1: String
  // TODO: not confirmed
}
