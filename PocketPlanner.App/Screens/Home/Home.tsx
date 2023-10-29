import { BottomTabNavigationProp, createBottomTabNavigator } from '@react-navigation/bottom-tabs'
import { FontAwesome5 } from '@expo/vector-icons'
import { View } from 'react-native'
import Summary from './Summary'
import Extract from './Extract'
import { colors } from '../../colors'

const Tab = createBottomTabNavigator<HomeRoutes>()

export type HomeRoutes = {
  "Summary": undefined
  "Extract": undefined
}

export type HomeNavigation = BottomTabNavigationProp<HomeRoutes, 'Summary'>

export default function() {
  return (
    <View style={{ flex: 1 }}>
      <View style={{ flex: 1, backgroundColor: colors.main }} />
      <View style={{ flex: 35 }}>
        <Tab.Navigator backBehavior='history' screenOptions={{ headerShown: false }}>
          <Tab.Screen name="Summary" component={Summary} options={{
            tabBarIcon: () => <FontAwesome5 name="receipt" size={24} color="black" />
          }} />
          <Tab.Screen name="Extract" component={Extract} options={{
            tabBarIcon: () => <FontAwesome5 name="file-import" size={24} color="black" />
          }} />
        </Tab.Navigator>
      </View>
    </View>
  )
}
