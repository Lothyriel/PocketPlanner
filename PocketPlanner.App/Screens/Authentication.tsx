import { View, StyleSheet, Button, Text } from 'react-native'
import * as WebBrowser from 'expo-web-browser'
import * as Google from 'expo-auth-session/providers/google'
import { useEffect, useState } from 'react'
import { jwtDecode } from 'jwt-decode'
import { get, set } from '../extensions'
import { TokenResponse } from 'expo-auth-session'
import { useNavigation } from '@react-navigation/native'
import { AppNavigation } from '../App'

WebBrowser.maybeCompleteAuthSession()

type UserInfo = {
  email: string,
  name: string,
  picture: string,
  locale: string
}

const GOOGLE_CONFIG = {
  clientId: '824653628296-ahr9jr3aqgr367mul4p359dj4plsl67a.apps.googleusercontent.com',
  iosClientId: '824653628296-5a4hseol33ep0vvo5tg29m39ib4src71.apps.googleusercontent.com',
  androidClientId: '824653628296-g4ij9785h9c1gkbimm5af42o4l7mket3.apps.googleusercontent.com'
}

export default function() {
  const navigation = useNavigation<AppNavigation>()

  const [token, setToken] = useState<TokenResponse | null>(null)
  const [user, setUser] = useState<UserInfo | null>(null)
  const [request, response, promptAsync] = Google.useIdTokenAuthRequest(GOOGLE_CONFIG)

  useEffect(() => {
    const fetchUserData = async () => {
      const userToken = await get<TokenResponse>('userToken')

      if (userToken?.idToken) {
        setUser(jwtDecode<UserInfo>(userToken.idToken))

        navigation.navigate('Home')
      }

      setToken(userToken)
    }

    fetchUserData();
  }, []);

  useEffect(() => {
    if (!response) {
      return
    }

    switch (response.type) {
      case 'success':
        const auth = response.authentication

        if (!auth) {
          throw "Success without auth"
        }

        if (!auth.idToken) {
          throw "Auth without token"
        }

        setUser(jwtDecode<UserInfo>(auth.idToken))

        set("userToken", auth)

        break
    }
  }, [response])

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Login com Google</Text>
      <Button title='Entrar' disabled={!request} onPress={() => promptAsync()} />
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
  }
})
