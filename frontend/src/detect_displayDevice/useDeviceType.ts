import { useMediaQuery } from '@mui/material'

export function useIsDesktop() {
  return useMediaQuery('(min-width:1024px)')
}
