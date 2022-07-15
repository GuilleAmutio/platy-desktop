import { Action, AnyAction, ThunkAction, ThunkDispatch } from "@reduxjs/toolkit";

export type PlatyAction<Payload> = Action & {
    payload: Payload
};

export type PlatyReducer<State, Payload> = (state: State, action: PlatyAction<Payload>) => any
export type PlatyReducerWithoutAction<State> = (state: State) => any
export type PlatyThunkDispatch<State> = ThunkDispatch<State, any, AnyAction>
export type PlatyThunkAnyAction<S> = AnyAction | ThunkAction<any, S, any, AnyAction>
