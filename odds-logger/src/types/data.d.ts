export type BasicDataType = {
  key: string
  index: number
  id: number
}

export type SelectType = {
  label: string
  value?: number
  key?: number
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
  home_win_start: string
  draw_start: string
  away_win_start: string
  home_win_end: string
  draw_end: string
  away_win_end: string
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
  predict_game_result: string
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
  predict_result: string
  year: string
  round: string
  note: string
  time: string
}

// odds type
export interface OddsBasicType {
  id: number
  match_id: number
}

// odds data from form
export interface OddsType {
  id: number
  bookmaker: SelectType
  bookmaker_id: number
  bookmaker_name: SelectType | string
  home_win_start: string
  home_win_end: string
  draw_start: string
  draw_end: string
  away_win_start: string
  away_win_end: string
}

// build odds data type
export interface OddsSubmitType extends OddsBasicType {
  bookmaker_id: number
  bookmaker_name: string
  home_win_start: string
  home_win_end: string
  draw_start: string
  draw_end: string
  away_win_start: string
  away_win_end: string
}

export interface OddsUpdateDataType extends OddsSubmitType {
  key: number
  name: number
  isListField: boolean
  fieldKey: number
}

export interface MatchOddsFormType {
  leagueInfo: SelectType
  home_team: SelectType
  away_team: SelectType
  game_time: MomentInput
  game_year: string
  game_round: string
  game_result: string
  predict_game_result: string
  history_note: string
  note: string
  match_id: number
  odds: OddsType[]
}
