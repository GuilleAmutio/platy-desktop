import { IClient, IRequest } from "./iclient"

const MockClient: IClient = {
    invoke: async <T>(request: IRequest) => {
        switch(request.name) {
            case "listContainers": return [
                { name: "gello", id: "123", state: "started"} 
            ] as any;  
        }
    }
}

export default MockClient
