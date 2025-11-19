# I simulated a nuclear plant, and let AI control it

## Tech stack

  Backend(Plant) - Rust
  Frontend - Yew/Js/TUI
  ML - PyTorch

## Part 1: Simulating

I need to code a minimal powerplant model. It should realistically simulate some easy processes.

The final output of the simulation should look something like this:

```json
{{
  "observation": {
    // Current absolute values
    "reactor-power": 12.39,
    "coolant-temp-in": 282.0,
    "coolant-temp-out": 315.0,
    
    // Rates of change (deltas per timestep)
    "reactor-power-rate": 0.15,      // MW/s - power is rising
    "coolant-temp-in-rate": -0.5,    // °C/s - cooling down
    "coolant-temp-out-rate": 2.3,    // °C/s - heating up fast!
    "pipeline-1-pressure-rate": -0.05  // MPa/s - pressure dropping
    
    "pipeline-1-temp": 211.0,
    "pipeline-1-temp-rate": 0.1,
    "pipeline-1-pressure": 31.8,
  }
```
Using delta values too to help my AI model. "the temperature is 315°C" is an OK statement, but the addition of "rising at 2.3 °C/s" would change everything.

This will give me a solid structure to export the data to the AI model in the future, and make the model somewhat
stateless (maybe just less state. Would the model consume less power?)

The simulation will consist of several services. 
1 - The actual plant.
2 - Service that consumes our electricity and gives us demands (ex: on weekends the city's power consumption is
bigger(?))

## Part 2: Connecting to the externals

Simulating is good, but I'll need to control the values in real time. I'll first use the request-responce model, and
then reconfigure it to be WebSockets.
Request:

```json
{
  "action":{
    "rod-insertion": -13 # "move rods DOWN by 13 units"  }
}
```

Responce:

```json
{
  "observation":{
    "reactor-power" : 12.39,
    "coolant-temp-in": 282.0,
    "coolant-temp-out": 315.0,
    "pipeline-1-temp": 211.0,
    "pipeline-1-pressure": 31.8,
  },
  "reward": -10.0, #Negative - bad: Reactor meltdown
  "terminated": false, #Are we still alive?
  "truncated": false, #Did we hit the maximum timesteps?
  "info": {
    "timestamp": 1.0,
    "warnings": ["coolant_temp_high"],
    "metrics": {
      "power stability": 0.95,
      "efficiency": 0.88
    }
  }
}
```

As seen, we have quite a few things to track. The legend is that we're powering a singular power control station, so the
end result should be this station's satisfaction of our deeds, given in precentage. In the future, I could increase the
quantity of the control stations.

## Part 3: Training

I'll hook up the PyTorch model, and train it for a couple of days on 2-4 cores.
The algorithm will be PPO.

### Training loop

```markdown
For 10,000 episodes:
  1. Reset reactor to starting conditions
  
  2. For each second of simulation:
     - AI observes: temp=315°C, pressure=31.8 MPa, power=12.39 MW
     - AI decides: "Move control rods down by 5 units"
     - Simulation runs for 1 second
     - New temp=313°C (good!), reward = +1
  
  3. After 10 minutes (or reactor melts down):
     - AI reviews everything it did
     - Updates its neural network:
       * "When temp was 315°C and rising, moving rods down was GOOD"
       * "When I moved rods too fast earlier, that was BAD"
  
  4. Repeat with slightly better strategy
```

Whether we punish AI or give him a cookie will be decided on the Python side.

## Current problems

  1. I'm ass at coding
  2. Where do I search the correct data, so I know the simulation is working properly?
