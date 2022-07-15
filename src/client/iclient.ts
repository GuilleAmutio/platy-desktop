export interface IRequest {
    name: string,
    payload: any
}

export interface IClient {
    invoke<T>(requst: IRequest): Promise<T>
}