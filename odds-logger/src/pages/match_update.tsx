import { message } from 'antd'
import { useParams } from 'react-router-dom'
import MatchInfo from '../components/match_info'

function MatchUpdate() {
  const { id } = useParams<{ id: string }>()
  const [messageApi, contextHolder] = message.useMessage()

  return (
    <>
      {contextHolder}
      <MatchInfo is_add={false} is_update={true} messageApi={messageApi} match_id={id} />
    </>
  )
}

export default MatchUpdate
