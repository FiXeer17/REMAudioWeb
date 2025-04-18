import { RouterProvider } from 'react-router-dom'
import { useIsDesktop } from './useDeviceType.ts'
import { DesktopRouter } from '../devicesRoutes/desktopRoute.tsx'
import { MobileRouter } from '../devicesRoutes/mobileRoute.tsx'

export function RouterSelector() {
  const isDesktop = useIsDesktop();
  return <RouterProvider router={isDesktop ? DesktopRouter : MobileRouter} />
}