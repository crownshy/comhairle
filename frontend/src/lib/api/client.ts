import {createApiClient as createApi} from "./api"

export const createApiClient = (baseUrl:string, authToken:string | undefined, source:string)=>{

  let api = createApi(baseUrl,{
    axiosConfig:{
      withCredentials:true
    }
  })

	api.axios.interceptors.request.use(config=>{ 
	  if(source==="server"){
  	  if(authToken){
    		config.headers['Cookie'] = `auth-token=${authToken}`;
  		}
		}
		return config
	})

	return api  
}

export const apiClient = createApiClient("/api", null , "client")
