import axios from 'axios';
import COMPANIES from './mock/explore';
import React, {useState} from 'react';
import {SearchBar} from 'react-native-elements';
import {NavigationContainer} from '@react-navigation/native';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import MaterialCommunityIcons from 'react-native-vector-icons/MaterialCommunityIcons';
import {
  SafeAreaView,
  StyleSheet,
  Text,
  View,
  FlatList,
  Image,
  TextInput,
  TouchableOpacity,
} from 'react-native';

const Tab = createBottomTabNavigator();

const Company = ({companyData}) => {
  return (
    <View style={styles.item}>
      <Image style={styles.logo} source={companyData.image} />
      <View style={styles.itemtext}>
        <Text style={styles.title}>{companyData.vaccine}</Text>
        <Text style={styles.subtitle}>{companyData.name}</Text>
        <Text style={styles.subtitle}>{companyData.location}</Text>
        <Text style={{color: 'blue', paddingTop: 5}}>Make Appointment</Text>
      </View>
    </View>
  );
};

const HomeScreen = () => {
  const [searchText, setSearchText] = useState('');

  const renderItem = ({item}) => <Company companyData={item} />;

  return (
    <SafeAreaView>
      <View>
        <SearchBar
          placeholder="Search Vaccine"
          onChangeText={text => setSearchText(text)}
          value={searchText}
          platform="ios"
        />
        <FlatList
          data={COMPANIES}
          renderItem={renderItem}
          keyExtractor={item => item.id}
        />
      </View>
    </SafeAreaView>
  );
};

const ValidateScreen = () => {
  const [validateText, setValidateText] = useState(
    'Please provide some details to verify your vaccine is legitamate',
  );
  const [validated, setValidated] = useState(false);
  const [isValid, setIsValid] = useState(false);

  const [company, setCompany] = useState('');
  const [vaccine, setVaccine] = useState('');

  const handleValidation = async () => {
    axios
      .get(
        `http://localhost:8000/block/validate/${company.toLowerCase()}/${vaccine.toLowerCase()}`,
      )
      .then(res => {
        setValidated(true);

        const {is_valid} = res.data;
        setIsValid(is_valid);

        if (is_valid) {
          setValidateText('You are safe to continue with your vaccination!');
        } else {
          setValidateText(
            'Please do no proceed with your vaccination at this location. This vaccine was not authorized.',
          );
        }
      });
  };

  return (
    <View
      style={{
        justifyContent: 'center',
        alignItems: 'center',
        flex: 1,
      }}>
      <Text style={{fontSize: 40}}>
        {validated ? (isValid ? '✅' : '❌') : null}
      </Text>
      <Text style={{width: 220, textAlign: 'center', fontSize: 17}}>
        {validateText}
      </Text>
      <TextInput
        style={styles.input}
        placeholder="Provider Name"
        onChangeText={setCompany}
        value={company}
      />
      <TextInput
        style={styles.input}
        placeholder="Vaccine Name/ID"
        onChangeText={setVaccine}
        value={vaccine}
      />
      <TouchableOpacity style={styles.button} onPress={handleValidation}>
        <Text
          style={{
            color: 'white',
            fontSize: 18,
            textAlign: 'center',
            marginTop: 6,
          }}>
          Validate!
        </Text>
      </TouchableOpacity>
    </View>
  );
};

const App = () => {
  return (
    <NavigationContainer>
      <Tab.Navigator initialRouteName="Home">
        <Tab.Screen
          name="Home"
          component={HomeScreen}
          options={{
            headerShown: false,
            tabBarIcon: ({color, size}) => (
              <MaterialCommunityIcons name="home" color={color} size={size} />
            ),
          }}
        />
        <Tab.Screen
          name="Validate"
          component={ValidateScreen}
          options={{
            headerShown: false,
            tabBarIcon: ({color, size}) => (
              <MaterialCommunityIcons name="check" color={color} size={size} />
            ),
          }}
        />
      </Tab.Navigator>
    </NavigationContainer>
  );
};

const styles = StyleSheet.create({
  item: {
    display: 'flex',
    flexDirection: 'row',
    backgroundColor: '#e0e0e0',
    padding: 20,
    marginVertical: 8,
    marginHorizontal: 16,
    borderRadius: 10,
  },
  itemtext: {
    paddingLeft: 10,
    display: 'flex',
    flexDirection: 'column',
  },
  title: {
    fontSize: 22,
  },
  subtitle: {
    fontSize: 14,
  },
  logo: {
    borderRadius: 50,
    borderWidth: 1,
    borderColor: '#fff',
    width: 50,
    height: 50,
    marginTop: 15,
  },
  input: {
    height: 40,
    width: 220,
    margin: 12,
    borderColor: '#cccaca',
    borderWidth: 1,
    borderRadius: 15,
    padding: 10,
  },
  button: {
    width: 100,
    height: 40,
    borderRadius: 20,
    backgroundColor: '#e8483c',
  },
});

export default App;
