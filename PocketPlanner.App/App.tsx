import { NavigationContainer } from '@react-navigation/native'
import { StackNavigationProp, createStackNavigator } from '@react-navigation/stack'
import Home from './Screens/Home/Home'
import Authentication from './Screens/Authentication'

const Stack = createStackNavigator<AppRoutes>()

export type AppNavigation = StackNavigationProp<AppRoutes, 'Authentication'>

export type AppRoutes = {
  "Home": undefined,
  "Authentication": undefined
}

export default function App() {
  return (
    <NavigationContainer>
      <Stack.Navigator screenOptions={{ headerShown: false }}>
        <Stack.Screen name="Authentication" component={Authentication} />
        <Stack.Screen name="Home" component={Home} />
      </Stack.Navigator>
    </NavigationContainer>
  )
}
