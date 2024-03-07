import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

import java.util.Scanner

import java.util.Scanner

println('Enter the city name:')

Thread userInputThread = new Thread({
    def scanner = new Scanner(System.in)
    def Cityname = scanner.nextLine()
})

userInputThread.start()

try {
    userInputThread.join(20000) 
    if (userInputThread.isAlive()) {
        throw new InterruptedException("City name not found(timed out).")
    }
} catch (InterruptedException e) {
    println("Failed to provide city name within given time.")
    throw e
}

if (userInputThread.isAlive()) {
    println("Failed to provide city name within given time.")
    throw new InterruptedException("City name input timed out.")
}

response = WS.sendRequest(findTestObject('Try/GetDynamic', [('CityName') : Cityname]))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())

println('City Name: ' + Cityname)

println('API Response:')
println(response.getResponseBodyContent())

println('Weather Description: ' + result.weather[0].description)
