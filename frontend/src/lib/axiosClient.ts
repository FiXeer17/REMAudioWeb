import axios,{AxiosInstance, AxiosRequestConfig,AxiosResponse} from "axios";

const host = import.meta.env.VITE_WS_HOST
class AxiosInterceptor {
    private axiosInstance : AxiosInstance;

    public get: <T>(url: string, config?: AxiosRequestConfig) => Promise<AxiosResponse<T>>;
    public post: <T>(url: string, data?: any, config?: AxiosRequestConfig) => Promise<AxiosResponse<T>>;
    public put: <T>(url: string, data?: any, config?: AxiosRequestConfig) => Promise<AxiosResponse<T>>;
    public delete: <T>(url: string, config?: AxiosRequestConfig) => Promise<AxiosResponse<T>>;

    constructor(instanceConfig:AxiosRequestConfig = {}) {
      this.axiosInstance = axios.create({
        timeout:8000,
        ...instanceConfig,
      });
  
      this.axiosInstance.interceptors.request.use(
        (config) => {
          const accessToken = this.getAccessToken();
          if (accessToken) {
            config.headers.Authorization = `Bearer ${accessToken}`;
          }
          return config;
        },
        (error) => Promise.reject(error),
      );
  
      this.get = this.axiosInstance.get.bind(this.axiosInstance);
      this.post = this.axiosInstance.post.bind(this.axiosInstance);
      this.put = this.axiosInstance.put.bind(this.axiosInstance);
      this.delete = this.axiosInstance.delete.bind(this.axiosInstance);
    }
  
    getAccessToken() {
      return localStorage.getItem("accessToken");
    }
  }
  

  export const client = new AxiosInterceptor({
    baseURL: `http://${host}/`,
  });