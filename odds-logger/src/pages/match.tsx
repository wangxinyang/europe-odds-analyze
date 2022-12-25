import { message } from 'antd'

import MatchInfo from '../components/match_info'

function Match() {
  // message info
  const [messageApi, contextHolder] = message.useMessage()

  return (
    <>
      {contextHolder}
      <MatchInfo is_add={true} is_update={false} messageApi={messageApi} />
    </>
  )
}

export default Match
