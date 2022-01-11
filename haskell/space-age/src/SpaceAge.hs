module SpaceAge (Planet(..), ageOn) where

data Planet = Mercury
            | Venus
            | Earth
            | Mars
            | Jupiter
            | Saturn
            | Uranus
            | Neptune

ageOn :: Planet -> Float -> Float
ageOn planet seconds = do
                            let earth_days = seconds / 31557600
                            case planet of
                                Mercury -> earth_days / 0.2408467
                                Venus -> earth_days / 0.61519726
                                Earth -> earth_days
                                Mars -> earth_days / 1.8808158
                                Jupiter -> earth_days / 11.862615
                                Saturn -> earth_days / 29.447498
                                Uranus -> earth_days / 84.016846
                                Neptune -> earth_days / 164.79132
