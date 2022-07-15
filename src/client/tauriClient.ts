import { tauri } from "@tauri-apps/api";
import { IClient, IRequest } from "./iclient";

const tauriClient: IClient = {
    invoke<T>(request: IRequest){
        const result: Promise<T> = tauri.invoke(request.name, request.payload)
        return result;
    }
}