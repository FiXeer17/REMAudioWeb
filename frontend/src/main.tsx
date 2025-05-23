import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router-dom'
import { AppRouter } from './detect_displayDevice/Routes'

createRoot(document.getElementById('root')!).render(
      <RouterProvider router={AppRouter}/>
)
