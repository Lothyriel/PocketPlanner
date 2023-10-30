import { View, StyleSheet, Button, Text, Image } from 'react-native'
import * as WebBrowser from 'expo-web-browser'
import * as Google from 'expo-auth-session/providers/google'
import { useEffect, useState } from 'react'
import { StatusBar } from 'expo-status-bar'

WebBrowser.maybeCompleteAuthSession()

type User = {
  email: string,
  name: string,
  picture: string
}

export default function() {
  const [accessToken, setAccessToken] = useState<string | null>(null)
  const [user, setUser] = useState<User | null>(null)
  const [request, response, promptAsync] = Google.useIdTokenAuthRequest({
    clientId: '824653628296-ahr9jr3aqgr367mul4p359dj4plsl67a.apps.googleusercontent.com',
    iosClientId: '824653628296-5a4hseol33ep0vvo5tg29m39ib4src71.apps.googleusercontent.com',
    androidClientId: '824653628296-g4ij9785h9c1gkbimm5af42o4l7mket3.apps.googleusercontent.com'
  })

  useEffect(() => {
    if (!response) {
      return
    }

    alert(JSON.stringify(response))

    switch (response.type) {
      case 'success':
        const auth = response.authentication?.accessToken ?? null
        setAccessToken(auth)

        accessToken && fetchUserInfo()
        break
    }
  }, [response, accessToken])

  async function fetchUserInfo() {
    const params = {
      headers: {
        Authorization: `Bearer ${accessToken}`
      }
    }

    const response = await fetch('https://www.googleapis.com/userinfo/v2/me', params)

    const userInfo = await response.json()

    setUser(userInfo)
  }

  if (user) {
    return (
      <View style={styles.container}>
        <Text style={{ fontSize: 20, marginBottom: 10 }}> Seja Bem Vindo ✌ </Text>

        <Image
          source={{
            uri: user.picture,
            width: 70,
            height: 70,
          }}
          borderRadius={40}
        />

        <Text style={{ marginTop: 10, fontSize: 17 }} > {user.name} </Text>

        <Text style={{ marginBottom: 20 }} > {user.email} </Text>

        <Button title='Sair' onPress={() => setUser(null)} />

        <StatusBar style='auto' />
      </View>
    )
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Login com Google</Text>
      <Button title='Entrar' disabled={!request} onPress={() => promptAsync()} />
      <StatusBar style='auto' />
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
  title: {
    marginBottom: 10,
    fontSize: 20,
  },
})
