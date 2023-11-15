import * as SecureStore from 'expo-secure-store'

export async function get<T>(key: string) {
  const data = await SecureStore.getItemAsync(key)

  if (!data) {
    return null
  }

  return JSON.parse(data) as T
}

export async function set(key: string, value: unknown) {
  const data = JSON.stringify(value)
  await SecureStore.setItemAsync(key, data)
}
