import { createSlice } from "@reduxjs/toolkit";
import { listContainers } from "./thunks";
import { 
    CONTAINERS_STORE_NAME, 
    INITIAL_STATE 
} from "./constants";
import { 
    _beginListContainers, 
    _containersListed, 
    _toggleView
} from "./reducers";

export * from "./types"
export { CONTAINERS_STORE_NAME } from "./constants"

const slice = createSlice({
    name: CONTAINERS_STORE_NAME,
    reducers: {
        toggleView: _toggleView
    },
    initialState: INITIAL_STATE,
    extraReducers: builder => {
        builder.addCase(listContainers.pending, _beginListContainers);
        builder.addCase(listContainers.fulfilled, _containersListed);
    }
});

// reducer actions
export const {
    toggleView
} = slice.actions

// thunk actions
export {
    listContainers
}

export default slice;