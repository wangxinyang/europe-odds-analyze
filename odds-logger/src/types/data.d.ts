type BasicDataType = {
  key?: string
  index?: number
  id: number
}

type SelectType = {
  label: string
  value?: number
  key?: number
}

export interface DataType extends BasicDataType {
  name: string
  note: string
}

export interface BookMakerDataType extends DataType {
  url: string
}

export interface TeamDataType extends DataType, BasicDataType {
  league_id: number
}

// define game record query page table type
export interface MatchInfoTableType extends BasicDataType {
  league_name: string
  vs: string
  year: string
  round: string
  result: string
  predict_result: string
  time: string
  note: string
}

type CommonMatchInfo = {
  game_year: string
  game_round: string
  game_result: string
  predict_game_result: string
  history_note: string
  note: string
}

// define match info data of form
export interface MatchInfoFormType extends CommonMatchInfo {
  leagueInfo: SelectType
  home_team: SelectType
  away_team: SelectType
  game_time: MomentInput
  note: string
  match_id: number
  odds: OddsType[]
}

// define match info data type from backend
export interface MatchInfoDataType extends CommonMatchInfo {
  id: number
  league_id: number
  league_name: string
  home_team_id: number
  away_team_id: number
  home_team: string
  away_team: string
  game_time: string
  oddsInfo: OddsDataType[]
}

type CommonOddsInfo = {
  id: number
  bookmaker_id: number
  bookmaker_name: SelectType | string
  home_win_start: string
  draw_start: string
  away_win_start: string
  home_win_end: string
  draw_end: string
  away_win_end: string
}

// define odds info data type from backend
export interface OddsDataType extends BasicDataType, CommonOddsInfo {
  match_id: number
  note?: number
}

// define odds data of form
export interface OddsFormType extends CommonOddsInfo {
  bookmaker: SelectType
}
