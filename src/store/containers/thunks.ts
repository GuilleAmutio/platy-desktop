import { createAsyncThunk } from "@reduxjs/toolkit";
import { IClient } from "client/iclient";
import { Container } from "./types";
import { CONTAINERS_STORE_NAME } from "./constants";


type PlatyThunkOptions = {extra: {client: IClient}}

export const listContainers = createAsyncThunk<Container[], undefined, PlatyThunkOptions>(
    `${CONTAINERS_STORE_NAME}]/listContainers`, 
    (_, { extra }) => {
        const { client } = extra
        const containers: Promise<Container[]> = client.invoke({
            name    : "listContainers",
            payload : {}
        })

        return containers;
})
