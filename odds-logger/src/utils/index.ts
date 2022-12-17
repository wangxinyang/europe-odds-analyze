import { MessageInstance } from 'antd/es/message/interface'

const success = (messageApi: MessageInstance, content: string, duration: number) => {
  messageApi.open({
    type: 'success',
    content,
    duration,
  })
}

const error = (messageApi: MessageInstance, content: string, duration: number) => {
  messageApi.open({
    type: 'error',
    content,
    duration,
  })
}

export { success, error }
