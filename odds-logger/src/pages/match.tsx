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
}

export default Match
