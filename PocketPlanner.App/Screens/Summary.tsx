import { View, StyleSheet } from "react-native"
import { TextInput } from 'react-native-paper';
import { colors } from "../colors"

export default function() {
  return (
    <View style={styles.container}>
      <View style={styles.search}>
        <Search />
      </View>
      <View style={styles.invoices}></View>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: 'white'
  },
  search: {
    flex: 1,
    padding: '4%',
  },
  invoices: {
    backgroundColor: 'grey',
    flex: 12,
  }
})

function Search() {
  const lightGrey = 'rgb(239, 240,244)'
  const grey = 'rgb(139, 139, 139)'

  const styles = StyleSheet.create({
    text: {
      flex: 1,
      fontSize: 20,
      backgroundColor: 'white'
    }
  })

  return (
    <TextInput
      style={styles.text}
      placeholder={"Buscar item"}
      onChangeText={() => { }}
      mode="outlined"
      activeOutlineColor={colors.main}
      outlineColor={lightGrey}
      placeholderTextColor={grey}
      outlineStyle={{ borderRadius: 10, borderWidth: 3 }}
      right={<TextInput.Icon icon="magnify" color={grey} />}
    />
  )
}
