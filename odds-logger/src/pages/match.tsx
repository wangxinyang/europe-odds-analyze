import { message } from 'antd'

import MatchInfo from '../components/match_info'

function Match() {
  // message info
  const [messageApi, contextHolder] = message.useMessage()

  return (
    <>
      {contextHolder}
      <MatchInfo is_add={true} messageApi={messageApi} />
    </>
  )
  // if (!leagueData[0]) {
  //   return <div>loading...</div>
  // } else {

  // }
}

export default Match
