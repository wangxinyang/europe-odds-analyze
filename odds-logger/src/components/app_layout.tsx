import { Layout, Menu, Breadcrumb } from 'antd'
import {
  LoginOutlined,
  BarChartOutlined,
  TagsOutlined,
  ToolOutlined,
  BankOutlined,
  OrderedListOutlined,
  PlusOutlined,
} from '@ant-design/icons'
import { Link, matchRoutes, Outlet, useLocation } from 'react-router-dom'
import { useEffect, useState } from 'react'
import { routes } from '../routes'

const { SubMenu } = Menu
const { Header, Content, Sider } = Layout

function AppLayout() {
  const location = useLocation()

  const [defaultSelectedKeys, setDefaultSelectedKeys] = useState<string[]>([])
  const [defaultOpenKeys, setDefaultOpenKeys] = useState<string[]>([])
  const [isInit, setIsInit] = useState<Boolean>(false)

  useEffect(() => {
    const routers = matchRoutes(routes, location.pathname)

    const pathArr: string[] = []
    if (routers !== null) {
      routers.forEach((item) => {
        const path = item.route.path
        if (path) {
          pathArr.push(path)
        }
      })
    }
    setDefaultSelectedKeys(pathArr)
    setDefaultOpenKeys(pathArr)
    setIsInit(true)
  }, [location.pathname])

  if (!isInit) {
    return null
  }

  return (
    <>
      <Layout style={{ minHeight: '100vh' }}>
        <Header className="header">
          <div className="logo" />
        </Header>
        <Layout>
          <Sider width={200} className="site-layout-background">
            <Menu
              mode="inline"
              defaultSelectedKeys={defaultSelectedKeys}
              defaultOpenKeys={defaultOpenKeys}
              style={{ height: '100%', borderRight: 0 }}>
              <SubMenu key="/" icon={<LoginOutlined />} title="赔率管理">
                <SubMenu key="1" icon={<BarChartOutlined />} title="比赛记录">
                  <Menu.Item key="5" icon={<OrderedListOutlined />}>
                    <Link to="/">查询</Link>
                  </Menu.Item>
                  <Menu.Item key="6" icon={<PlusOutlined />}>
                    <Link to="/add_match_info">新增</Link>
                  </Menu.Item>
                </SubMenu>
                <Menu.Item key="2" icon={<ToolOutlined />}>
                  <Link to="/league">联赛</Link>
                </Menu.Item>
                <Menu.Item key="3" icon={<TagsOutlined />}>
                  <Link to="/team">球队</Link>
                </Menu.Item>
                <Menu.Item key="4" icon={<BankOutlined />}>
                  <Link to="/bookmaker">博彩公司</Link>
                </Menu.Item>
              </SubMenu>
            </Menu>
          </Sider>
          <Layout style={{ padding: '0 24px 24px' }}>
            <Breadcrumb style={{ margin: '16px 0' }}>
              <Breadcrumb.Item>Home</Breadcrumb.Item>
              <Breadcrumb.Item>List</Breadcrumb.Item>
              <Breadcrumb.Item>App</Breadcrumb.Item>
            </Breadcrumb>
            <Content
              className="site-layout-background"
              style={{
                padding: 24,
                margin: 0,
                minHeight: 280,
              }}>
              <Outlet />
            </Content>
          </Layout>
        </Layout>
      </Layout>
    </>
  )
}

export default AppLayout
