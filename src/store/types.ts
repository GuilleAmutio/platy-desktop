import { Action } from "@reduxjs/toolkit";

export type PlatyAction<Payload> = Action & {
    payload: Payload
};

export type PlatyReducer<State, Payload> = (state: State, action: PlatyAction<Payload>) => any
export type PlatyReducerWithoutAction<State> = (state: State) => any