import { BottomTabNavigationProp, createBottomTabNavigator } from '@react-navigation/bottom-tabs'
import { NavigationContainer } from '@react-navigation/native'
import { FontAwesome5 } from '@expo/vector-icons'
import { View } from 'react-native'
import { colors } from './colors'
import Summary from './Screens/Summary'
import Extract from './Screens/Extract'

const Tab = createBottomTabNavigator<Routes>()

export default function App() {
  return (
    <View style={{ flex: 1 }}>
      <View style={{ flex: 1, backgroundColor: colors.main }} />
      <View style={{ flex: 35 }}>
        <NavigationContainer>
          <Tab.Navigator backBehavior='history' screenOptions={{ headerShown: false }}>
            <Tab.Screen name="Summary" component={Summary} options={{
              tabBarIcon: () => <FontAwesome5 name="receipt" size={24} color="black" />
            }} />
            <Tab.Screen name="Extract" component={Extract} options={{
              tabBarIcon: () => <FontAwesome5 name="file-import" size={24} color="black" />
            }} />
          </Tab.Navigator>
        </NavigationContainer>
      </View>
    </View >
  )
}

export type Routes = {
  "Summary": undefined
  "Extract": undefined
}

export type Navigation = BottomTabNavigationProp<Routes, 'Summary'>
