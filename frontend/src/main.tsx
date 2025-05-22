import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router-dom'
import { AppRouter } from './detect_displayDevice/Routes'
import { loadConfig } from './config';
import { createClient } from './lib/axiosClient';

loadConfig().then(()=>{
      createClient();
      createRoot(document.getElementById('root')!).render(
      <RouterProvider router={AppRouter}/>
      )
})

