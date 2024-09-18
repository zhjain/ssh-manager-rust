function isTauriError(error: unknown): error is TauriError {
    return error instanceof Error && '__tauriModule' in error
}

export { isTauriError }
