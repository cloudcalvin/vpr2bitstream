
#[macro_use]
use global::*;
use errors::*;
use types::*;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};


// struct RouteGraph {
//   name : String,
//   source : String,
//   child : Graph,
//   weight_total : u32,
// }




// // impl RouteNode for RouteEvent // actually make RouteEvent trait extend RouteNode
// pub trait RouteEvent : RouteNode{}

// pub struct SyncSourceEvent{
//   // type : RouteEventEnum,
//   point : Point,
//   // source : Source,
//   // track : Track, 
//   // rr_type : NodeMetaType, //should be CHANX/CHANY
// }

// impl RouteEvent for SyncSourceEvent{}
// impl RouteNode for SyncSourceEvent{}

// pub struct AsyncSourceEvent{
//   point : Point,
//   // type : RouteEventEnum,
//   // source_location : Point,
//   // track_location : Point,
// }

// impl RouteEvent for AsyncSourceEvent{
  
// }
// impl RouteNode for AsyncSourceEvent{}



// pub struct SyncSinkEvent{
//   point : Point,
// }
// impl RouteEvent for SyncSinkEvent{}
// impl RouteNode for SyncSinkEvent{}

// pub struct AsyncSinkEvent{
//   point : Point,
// }
// impl RouteEvent for AsyncSinkEvent{}
// impl RouteNode for AsyncSinkEvent{}


// pub struct TrackSwitchEvent{
//   point : Point,
// }

// impl RouteEvent for TrackSwitchEvent{}
// impl RouteNode for TrackSwitchEvent{}

// pub type RouteEventDelayPairList = Vec<(RouteDelay,Vec<(EventDelay,RoutingEvent)>)>; 
/// Node in the routing event graph
/// TODO : re do this list based approach, changing it to be graph based, like an AST. (delay,event) -> Nodes
/// get_route_timings

pub type DelayEventGraph = Vec<(NetName,Vec<(RouteDelay,Vec<(Delay,RouteEventEnum)>)>)>;
pub type NetDelayEventGraph = Vec<(RouteDelay,Vec<(Delay,RouteEventEnum)>)>;
pub type RouteDelayEventGraph = Vec<(Delay,RouteEventEnum)>;
pub type NamedDelayEventGraphList = Vec<(ModelName,DelayEventGraph)>;

pub fn build_delay_event_graph<'a>( models : &'a Vec<Model>, nets: &'a Vec<RouteNet>, place_map : &'a PlacementMap )
                            -> Result<NamedDelayEventGraphList> {

  
  let mut delay_event_graphs = Vec::new();
  vv_timing_println!("{}", Red.bold().paint("TIMING ANALYSIS : Building delay event graphs"));
  
  for model in models {
    let mut model_delay_event_graph = Vec::new();
    let model_latches = model.latched.iter().map(|x| &x.0).collect::<Vec<_>>();
    vv_timing_println!("{}", Red.bold().paint(format!("model latches {:#?} ",model_latches)));
    
    // let delays = Vec::new()
    // let events = Vec::new()
    for net in nets {
       vv_timing_println!("TIMING ANALYSIS : Net : {:#?} ",net.name);                 
      
      let mut net_route_list : Vec<(RouteDelay,Vec<(Delay,RouteEventEnum)>)> = Vec::new();
      let source_name = place_map.get(&net.src.0).expect("Net source could not be found in placement map.");
      //if source is pad or latched make the first element in the first route SyncSourceEvent
      // else AsyncSourceEvent
      let (source_delay,source_event) = if( net.src.1 == NodeMetaType::Pad ) {
        // PAD -> Latched
        vv_timing_println!("TIMING ANALYSIS : Found pad {:?}",net.src);
        (
          (*PARAMETERS).get("FPGA_DELAY_PAD_EXIT")
            .expect("Parameter 'FPGA_DELAY_PAD_EXIT' has not been declared")
            .parse::<u32>()
            .expect("Could not parse parameter to Int.")
          , 
          RouteEventEnum::SyncSourceEvent( RouteEvent{ point:net.src.0.clone() } )  
        )
      }else if ( model_latches.contains(&source_name)){
        // Latched
        vv_timing_println!("TIMING ANALYSIS : Found Latched BLE {:?}",net.src);      
        (
          (*PARAMETERS).get("FPGA_DELAY_BLE_EXIT")
            .expect("Parameter 'FPGA_DELAY_BLE_EXIT' has not been declared")
            .parse::<u32>()
            .expect("Could not parse parameter to Int.")
          , 
          RouteEventEnum::SyncSourceEvent( RouteEvent{ point:net.src.0.clone() } ) 
        ) 
      }else{
        // Not Latched
        vv_timing_println!("TIMING ANALYSIS : Found Unlatched BLE {:?}",net.src); 
        (
          (*PARAMETERS).get("FPGA_DELAY_BLE_EXIT")
            .expect("Parameter 'FPGA_DELAY_BLE_EXIT' has not been declared")
            .parse::<u32>()
            .expect("Could not parse parameter to Int.")
          , 
          RouteEventEnum::AsyncSourceEvent( RouteEvent{ point:net.src.0.clone() } )
        ) 
      };

      let route_tree : &Vec<Route> = net.route_tree.as_ref();

      // check wether new branch starts fresh from main source or has tracks to work from.
      for branch in route_tree {

        // if the branch starts from the source, then start a route and add it to the net_route        
        let mut route : Vec<(Delay,RouteEventEnum)> = 
          if branch.from_opin {
              
              vv_timing_println!("TIMING ANALYSIS : Creating new route."); 
            
              let mut new_route = Vec::new();
              let entry_delay = 0u32;
              let entry_event = RouteEventEnum::TrackEntryEvent(RouteEvent{point:branch.tracks[0].xy.clone()});

              new_route.push((source_delay.clone(),source_event.clone()));              
              new_route.push((entry_delay,entry_event));
        
              new_route

          }else{
            // else if subsource, go backward through the routes, find the route that has this point.
            // then take that route up to that point and copy it, adding the copy to the net_route_list, as 
            // a new route. 
            
            let mut new_route = Vec::new();          
            vv_timing_println!("TIMING ANALYSIS : Creating route from prior route."); 
            
            'outer: for &( ref prior_delay, ref prior_route ) in &net_route_list{

              let prior : &Vec<(Delay,RouteEventEnum)> = prior_route;

              for (i,&( ref delay, ref event)) in prior.into_iter().enumerate(){

                let point = event.point();
                vv_timing_println!("TIMING ANALYSIS : Comparing points ({:?}) and ({:?})",&point,branch.tracks[0].xy); 
                
                if point == branch.tracks[0].xy{

                  vv_timing_println!("TIMING ANALYSIS : Subsource found in prior route:");
                  vv_timing_println!("TIMING ANALYSIS : {{[");
                  
                  
                  for &( _, ref e) in &prior[0..i+1]{

                    if(e != event){

                      vv_timing_println!("TIMING ANALYSIS :   {:?} " , e.point());

                    }else{

                      vv_timing_println!("TIMING ANALYSIS :   {:?} <<< branch point " , e.point());

                    }
                  }

                  vv_timing_println!("TIMING ANALYSIS : ]}}");
                  
                  // vv_timing_println!("TIMING ANALYSIS : Subsource at point {:} used for slicing prior route."); 
                  // new_route = prior[0..i+1].iter().collect::<Vec<(Delay,RouteEventEnum)>>();
                  new_route = prior[0..i+1].to_vec();
                  // new_route.from_slice(&prior[0..i+1]);
                  println!("TIMING ANALYSIS : Found Branchpoint");
                  break 'outer
                }
              }
              println!("TIMING ANALYSIS : Trying next route for branchpoint");
            }
            if new_route.is_empty(){
              unreachable!("Creating new_route failed.")
            };

            new_route
          };

          

        vv_timing_println!("TIMING ANALYSIS : Seed events >>");        
         
        for &( ref delay, ref event) in route.iter(){
          vv_timing_println!("TIMING ANALYSIS : {:?} @({:?}) (Delay -> {:?})", event, event.point(),delay);         
        }


        // vv_timing_println!("TIMING ANALYSIS : TrackEntryEvent @ ({:?})", branch.tracks[0].xy);   
        vv_timing_println!("TIMING ANALYSIS : << ");                 

      
        
              
        vv_timing_println!("TIMING ANALYSIS : new events >> ");         

        // each track to the next there is a routing event : SWITCHEVENT, PASSTHROUGHEVENT
        for track in branch.tracks.iter().skip(1){

          let delay = (*PARAMETERS).get("FPGA_DELAY_TRACKSWITCH")
            .expect("Parameter 'FPGA_DELAY_TRACKSWITCH' has not been declared")
            .parse::<u32>()
            .expect("Could not parse parameter to Int.");
            
          let event = RouteEventEnum::TrackSwitchEvent(RouteEvent{point:track.xy.clone()});

          vv_timing_println!("TIMING ANALYSIS : TrackSwitchEvent @ ({:?}) (Delay -> {:?})",  track.xy,delay);   
          
          route.push((delay,event));
        }
        
        vv_timing_println!("TIMING ANALYSIS : << ");                 
        
        // EACH SINK has a SINKEVENT which could be a CONNECTIONBLOCK_EXIT. stick with SINKEVENT
        // SOURCE EVENTS are either from pad or ble but always go into Connectionblock.

        //get sink name
        let p : &Point = &branch.sink.as_ref().expect("Error: Branch has no sink").0;
        let sink_block_name = place_map.get(p)
          .expect(&format!("Error: Sink coords could not be used to find BLE.({:#?})>\n{:#?}\n<",&branch.sink,place_map));

        vv_timing_println!("TIMING ANALYSIS : Sink Found ");                 
        
        // vv_timing_println!("{}",Red.bold().paint(format!("{:#?}",model)));

        //take the sink, and check if its latched or pad. 
        let (sink_delay,sink_event) = if( branch.sink.as_ref().unwrap().1 == NodeMetaType::Pad ) 
          {          
            (
              (*PARAMETERS).get("FPGA_DELAY_PAD_ENTER")
                            .expect("Parameter 'FPGA_DELAY_PAD_ENTER' has not been declared")
                            .parse::<u32>()
                            .expect("Could not parse parameter to Int.")
            , 
              RouteEventEnum::SyncSinkEvent( RouteEvent{
                                              point:branch.sink.as_ref().unwrap().0.clone()
                                            }
              )
            )
          }else if ( model_latches.contains(&sink_block_name) ){            
            (
              (*PARAMETERS).get("FPGA_DELAY_BLE_ENTER")
                .expect("Parameter 'FPGA_DELAY_BLE_ENTER' has not been declared")
                .parse::<u32>()
                .expect("Could not parse parameter to Int.")
              , 
              RouteEventEnum::SyncSinkEvent(RouteEvent{point:branch.sink.as_ref().unwrap().0.clone()})
            )
          }else{

            
            (
              (*PARAMETERS).get("FPGA_DELAY_BLE_ENTER")
                .expect("Parameter 'FPGA_DELAY_BLE_ENTER' has not been declared")
                .parse::<u32>()
                .expect("Could not parse parameter to Int.")
              ,
              RouteEventEnum::AsyncSinkEvent(RouteEvent{point:branch.sink.as_ref().unwrap().0.clone()})
            )
          };

        vv_timing_println!("TIMING ANALYSIS : {:?} (Delay -> {:?})",sink_event,sink_delay); 
        vv_timing_println!("TIMING ANALYSIS : BRANCH END "); 
        
        route.push((sink_delay,sink_event));
        // Add vec of delay+event pairs into the route.
        let route_delay = route.iter().fold(0u32, | sum,&(delay,_) | sum + delay);
        net_route_list.push((route_delay,route));
      }    
      vv_timing_println!("TIMING ANALYSIS : NET END "); 
      
      model_delay_event_graph.push((net.name.clone(),net_route_list));
    }
    delay_event_graphs.push((model.name.clone(), model_delay_event_graph));
  }
  Ok(delay_event_graphs)  
}

pub type SynCritPathDelay = Delay;
pub type AsynCritPathDelay = Delay;
pub type CritPathDelays = (SynCritPathDelay,AsynCritPathDelay);

/// Get the longest BLE to BLE delay from the delay graph
/// 
/// This is done by scannin all route-delays in the graph for the longest route-delay. 
///
pub fn get_inter_ble_critical_path_delay<'a>( delay_event_graph : &'a NamedDelayEventGraphList) 
                                              -> Result<Delay>{
  let mut inter_ble_crit_path = 0;

  for &( ref model, ref model_delay_event_graph ) in delay_event_graph{
    for &( ref net_name, ref net_delay_event_graph ) in model_delay_event_graph{
      for &(ref route_delay,_) in net_delay_event_graph{
        if *route_delay > inter_ble_crit_path{
          inter_ble_crit_path = *route_delay;
        }
      }
    }
  }

  Ok( inter_ble_crit_path )
}

/// Get the longest latched-block to latched-block delay.
/// 
/// Returns the time in seconds of the longest path.
/// NOTE: this is purely usable in asynchronously flowing circuits.
///
/// This is done by scanning all LATCH to LATCH paths, through any non-latched blocks
/// for the over-all longest path.  
/// 
pub fn get_async_inter_latch_crit_path_delay<'a>( delay_event_graph : &'a NamedDelayEventGraphList, 
                                            place_map : &'a PlacementMap) 
                                            -> Result<Delay>{
  
  let mut inter_latch_crit_path = 0;  
  vv_timing_println!("{}", Red.bold().paint("TIMING ANALYSIS : Calculating async_inter_latch_crit_path_delay."));         

  //use breadth first search/accumulation to find to longest inter-latch-critical-path.  

  for &( ref model, ref model_delay_event_graph ) in delay_event_graph{
    for &( ref net_name, ref net_delay_event_graph ) in model_delay_event_graph{

      // only latched sources are used as starting points.
      // This does not need the model any more as the details are in the graph.
      let ref source_event =  net_delay_event_graph[0].1[0].1;
      vv_timing_println!("TIMING ANALYSIS : Starting at source : {:?}",source_event);         
      
      match source_event {        
        &RouteEventEnum::SyncSourceEvent(ref source_event_struct) => {
          
          // vv_timing_println!("TIMING ANALYSIS : {:?} @({:?}) (Delay -> {:?})", event, event.point(),delay);         
          
          for &( ref route_delay_sum, ref route_delays ) in net_delay_event_graph {
            let ref sink_event = route_delays.last().unwrap().1;

            match sink_event{
              
              // For every async path, find longest delay in all child nodes, recursively.
              &RouteEventEnum::AsyncSinkEvent(ref sink_event_struct) => {
                vv_timing_println!("TIMING ANALYSIS : Trying new source from sink : {:?}",sink_event);         

                // use sink to find the right net_delay_event_graph in the model_delay_event_graph;
                let sink_name = place_map.get(&sink_event_struct.point).unwrap();
                let ref sink_delay_graph : NetDelayEventGraph = model_delay_event_graph
                  .into_iter()
                  .filter(|&&(ref net_name, ref graph)| net_name == sink_name)
                  .collect::<Vec<_>>()
                  .get(0).unwrap().1; 

                // and pass this net_delay_event_graph into recursive function.
                        
                // for every sink which is not latched, explore it.
                let longest_multi_route_delay = async_inter_latch_crit_path_delay_helper( route_delay_sum,
                                                                                    &sink_delay_graph,
                                                                                    model_delay_event_graph,
                                                                                    place_map).unwrap();

                if longest_multi_route_delay > inter_latch_crit_path{
                  inter_latch_crit_path = longest_multi_route_delay; 
                }   
                vv_timing_println!("TIMING ANALYSIS : Longest delay was found to be : {:?}",inter_latch_crit_path);         
                
              },

              // for every latched route, compare the routes delay with the async routes(the cumulative delay).      
              &RouteEventEnum::SyncSinkEvent(ref sink_event_struct) => {
                if *route_delay_sum > inter_latch_crit_path{
                  inter_latch_crit_path = *route_delay_sum; 
                }         
              },
              _ => println!("no-op")//do nothing.
            }

            // for &( ref delay, ref event ) in route_delays{

            //   if route_delay > inter_ble_crit_path{
            //     inter_ble_crit_path = route_delay;
            //   }
            // }
          }
        }
        _ => continue
      }
    }
  }
  if inter_latch_crit_path != 0{
    Ok(inter_latch_crit_path)
  }else{
    Err("Error: Could not calculate delay(async inter-latch).".into())
  }

}


/// Recursive helper function.
/// 
fn async_inter_latch_crit_path_delay_helper<'a>( seed_delay : &'a Delay, 
                                                net_delay_event_graph : &'a NetDelayEventGraph,
                                                model_delay_event_graph : &'a DelayEventGraph, 
                                                place_map : &'a PlacementMap) 
                                                -> Result<Delay>{

  //now that we are in a new net_event_graph, look for the longest delay, and append it to the seed_delay and return.
  let mut this_longest_multi_route_delay = 0;

  for &( ref route_delay_sum, ref route_delays ) in net_delay_event_graph {
    let ref sink_event = route_delays.last().unwrap().1;
    let child_delay = match sink_event{
      &RouteEventEnum::AsyncSinkEvent(ref e) => {
        vv_timing_println!("TIMING ANALYSIS : Trying new source from sink : {:?}",sink_event);         
  
        // use sink to find the right net_delay_event_graph in the model_delay_event_graph;
        // and pass this net_delay_event_graph into recursive function.
        let sink_name = place_map.get(&e.point).unwrap();
        let ref sink_delay_graph : NetDelayEventGraph = model_delay_event_graph
            .into_iter()
            .filter(|&&(ref net_name, ref graph)| net_name == sink_name)
            .collect::<Vec<_>>()
            .get(0).unwrap().1; 
        
        // for every sink which is not latched, explore it.
        async_inter_latch_crit_path_delay_helper(route_delay_sum,&sink_delay_graph,model_delay_event_graph,place_map)?
      },

      // for every latched route, compare the routes delay with the async routes(the cumulative delay).      
      &RouteEventEnum::SyncSinkEvent(ref e) => {
        *route_delay_sum      
      },
      _ => 0
    };
    if child_delay > this_longest_multi_route_delay{
      this_longest_multi_route_delay = child_delay; 
    }   
    vv_timing_println!("TIMING ANALYSIS : Longest delay was found to be : {:?}",this_longest_multi_route_delay);         
    
  }
  Ok(*seed_delay + this_longest_multi_route_delay)
}


/// Get the longest latched-block to latched-block path in terms of the critical path delay count .
/// 
/// Returns the time in seconds of the longest path, where each link in the path has a delay equivalent
/// to the longest link in all paths(the critical path), as well as the total amount of links in the path.
///
/// This is done by scanning all LATCH to LATCH paths, through any non-latched blocks, for the 
/// path or paths with the largest amount of blocks, then multiplying that with the delay of the  
/// over-all critical inter-BLE critical path delay. This means that all of the paths with the same  
/// amount of BLE links, will have the same critical expanded-inter-latch/synchronised-inter-latch path delay
///
/// NOTE: this is purely usable in synchronous flowing circuits where each each and every block 
/// requires a clock in addition to the LATCHES that are clocked for state change synchronosiation.
/// 
pub fn get_sync_inter_latch_crit_path_delay<'a>(inter_ble_critical_path : Delay, delay_event_graph : &'a NamedDelayEventGraphList, place_map : &'a PlacementMap) -> Result<Delay>{
  
  let mut count_max = 0;  
  vv_timing_println!("{}", Red.bold().paint("TIMING ANALYSIS : Calculating sync_inter_latch_crit_path_delay.")); 

  //use breadth first search/accumulation to find to longest inter-latch-critical-path.  

  for &( ref model, ref model_delay_event_graph ) in delay_event_graph {
    for &( ref net_name, ref net_delay_event_graph ) in model_delay_event_graph {

      // only latched sources are used as starting points.
      // This does not need the model any more as the details are in the graph.
      let ref source_event =  net_delay_event_graph[0].1[0].1;
      
      match source_event {

        // for every sink which is not latched, explore it.
        &RouteEventEnum::SyncSourceEvent(ref source_event_struct) => {
          vv_timing_println!("TIMING ANALYSIS : Starting at source : {:?}",source_event_struct);         
          
          for &( ref route_delay_sum, ref route_delays ) in net_delay_event_graph {
            let ref sink_event = route_delays.last().unwrap().1;

            match sink_event{

              // use sink to find the right net_delay_event_graph in the model_delay_event_graph;
              // and pass this net_delay_event_graph into recursive function.
              &RouteEventEnum::AsyncSinkEvent(ref sink_event_struct) => {
             

                let sink_name = place_map.get(&sink_event_struct.point).unwrap();

                let ref sink_delay_graph : NetDelayEventGraph = model_delay_event_graph
                  .into_iter()
                  .filter(|&&(ref net_name, ref graph)| net_name == sink_name)
                  .collect::<Vec<_>>()
                  .get(0).unwrap().1; 

                // and pass this net_delay_event_graph into recursive function.        
                let longest_multi_route_count = sync_inter_latch_crit_path_count( &sink_delay_graph,
                                                                                  model_delay_event_graph,
                                                                                  place_map)
                                                                                  .unwrap();

                if longest_multi_route_count > count_max {
                  count_max = 1 + longest_multi_route_count; 
                }   
                vv_timing_println!("TIMING ANALYSIS : Longest path count was found to be : {:?}",count_max);         
                
              },
              // for every latched route, compare the routes delay with the async routes(the cumulative delay).
              &RouteEventEnum::SyncSinkEvent(ref sink_event_struct) => {
                if 1 > count_max{
                  count_max = 1; 
                }         
              },
              _ => println!("no-op")//do nothing.
            }

          }
        }
        _ => continue
      }
    }
  }
  if count_max != 0  && inter_ble_critical_path != 0 {
    Ok(count_max*inter_ble_critical_path)
  }else{
    Err("SEX Error: Could not calculate delay(sync inter-latch).".into())
  }

}

/// Recursive helper function.
/// 
fn sync_inter_latch_crit_path_count<'a>( net_delay_event_graph : &'a NetDelayEventGraph,
                                         model_delay_event_graph : &'a DelayEventGraph, 
                                         place_map : &'a PlacementMap) 
                                         -> Result<u32> {

  let mut count_max = 0;

  for &( ref route_delay_sum, ref route_delays ) in net_delay_event_graph {
    let ref sink_event = route_delays.last().unwrap().1;
    
    let longest_multi_route_count = match sink_event{

      // for every sink which is not latched, explore it.
      &RouteEventEnum::AsyncSinkEvent(ref e) => {
  
        // use sink to find the right net_delay_event_graph in the model_delay_event_graph;
        let sink_name = place_map.get(&e.point).unwrap();
        let ref sink_delay_graph : NetDelayEventGraph = model_delay_event_graph
            .into_iter()
            .filter(|&&(ref net_name, ref graph)| net_name == sink_name)
            .collect::<Vec<_>>()
            .get(0).unwrap().1; 

        // and pass this net_delay_event_graph into recursive function.        
        1 + sync_inter_latch_crit_path_count( &sink_delay_graph,
                                              model_delay_event_graph,
                                              place_map)?
      },

      // for every latched route, compare the routes delay with the async routes(the cumulative delay).
      &RouteEventEnum::SyncSinkEvent(ref e) => {
        1    
      },
      _ => 0
    };
    if longest_multi_route_count > count_max{
      count_max = longest_multi_route_count; 
    }   
  }
  Ok( count_max )
}

// /// Node in the routing event graph
// ///
// /// get_route_timings
// pub fn net_delay_graph<'a>( models : &'a Vec<Model>, nets: &'a Vec<RouteNet>, place_map : &'a Vec<PlacementMap> ) 
//                               // -> Result<HashMap<String,Vec<RoutingEvent>>> {
//                               // -> Result<Vec<NetName,(PathDelay,Vec<RoutingEvent>)>> {
//                             -> Result<Vec<(NetName,PathDelay,Vec<PathDelay,Vec<RoutingEvent>>)>> {

//   // NOTE : Route contains rr_nodes and a sink but have no "source" per se. 
//   // ALSO : TODO : branches are parts of a route tree. but every source->sink is a route. 
//   // Current implementation does not really make sense in that regard.
//   // let mut rr_event_lists : Vec<(String,Vec<RoutingEvent>)> = Vec::New();
//   // let mut rr_type_lists : Vec<(String,Vec<Route>)> = Vec::New();
//   // let mut net_delay_graph : Vec<NetName,(PathDelay,Vec<RoutingEvent>) = Vec::New();
//   let mut net_delay_graph : Vec<(NetName,PathDelay,Vec<PathDelay,Vec<RoutingEvent>>)> = Vec::New();

//   for model in models {
//     for net in nets {
//       if ( model.latched.contains(&net.name) ) || ( net.src.3 == NodeMetaType::Pad ) {
//         // let mut start_delay = (*PARAMETERS).get("FPGA_DELAY_PAD_ENTR
//  Y").expect("Parameter 'FPGA_DELAY_PAD_ENTRY' has not been declared")
//  .parse::<u32>() as u32;    
//         // let routes : &Vec<Route> = net.route_tree.as_ref();        
//        
//         // need to think about whet traversal means : need to propate the cumulative delay?

//         // For every route in the net, (source -> sink) get the delay of that route.
//         let (route_delay, route_events) = net_delay_graph_helper((*PARAMETERS).get("FPGA_DELAY_PAD_ENTR
//  Y").expect("Parameter 'FPGA_DELAY_PAD_ENTRY' has not been declared")
//  .parse::<u32>() as u32, models,net,place);
//
//         net_delay_graph.push((net.name,max(route_delay),route_delay,route_events));
//       }
//     }
//   }
  
//   Ok(net_delay_graph)
// }

// //recursive helper function to travers routing trees, to get delay.
// fn net_delay_graph_helper(delay_acc: PathDelay, models : &'a Vec<Model>, nets: &'a Vec<RouteNet>, place_map : &'a Vec<PlacementMap> )
//                           -> Result<Vec<(NetName,PathDelay,Vec<PathDelay,Vec<RoutingEvent>>)>> {
  
//   //get underlying routing tree
//   let routes : &Vec<Route> = net.route_tree.as_ref();

//   // if this route tree starts with a latched source or an input pad.
//   if ( model.latched.contains(&net.name) ) || ( net.src.3 == NodeMetaType::Pad ) {

//     let mut route_events = Vec::new();
//     let mut route_delay = (*PARAMETERS).get("FPGA_DEALY_PAD_ENTR
//  Y").expect("Parameter 'FPGA_DEALY_PAD_ENTRY' has not been declared")
//  .parse::<u32>() as u32;
//
//     for route in routes{
//       route_delay += (route.len as u32) * (*PARAMETERS .get("FPGA_DELAY_SINGLE_LEVEL_SWITCHa
//  s").expect("Parameter 'FPGA_DELAY_SINGLE_LEVEL_SWITCH' has not been declared")
//  .parse::<u32>() u32);
//       match route.sink.unwrap().3 {
//         NodeMetaType::Pad => {
//           //output port , thus latched on recieving end.
//           // take the delay until now and 
//         }
//         NodeMetaType::Pin => {
//           //latched BLE/CLB
//           //get BLE name from placement file.
//           let block_name = place_map.get(sink.0).unwrap();
//           if model.latched.contains(&block_name){
//             //latched thus return the delay count.
//           }else{
//             //travers the next ble's route trees ( forming breadth first graph weight accumulator )
//             //use the block name to get the next route tree from the nets and continue one with 
//             // the bredth first
//             // % EVERY TIME A SINK IS FOUND, THE PATH DELAY TO THE SINK IS RECORDED, THEN THE 
//             //  PATH DELAY UP TO THE SUBSOURCE MUST BE CALCULATED AND SENT WITH THE SUBSOURCE 

//             let mut next_net = RouteNet::default(); 
//             for (i,net) in nets.enumerate{
//               if net.name(block_name){
//                 next_net = nets[i];
//                 net_delay_graph.push(net_delay_graph_helper(models,next_net,place));
//               }
//             }

            
//           }
//         }
//       }
//       // source -> sink

//       // subsources -> sinks

//       route.last_mut().
//     }

//     // if latched end here.
//     // else travers that 

//     //since this is the output of a latch, there can only be one event.
    
//     // Add new route_evnt_list and give it BLE_ExitEvent
//     // routes.push((net.name, Vec::new()));

    
//     //get source - sink delays and push them to array.
//     //if an unlatched sink is found, take count and 

//     println!("Source found to be latched in {:} model", models[0].name);
//     route.

//   }else{ //this is part of a previous rr_event_list
//   }
// }

// fn graph_travers_until_latched( rr : &Vec<Route>, new_rr : &mut Vec<RouteEvent> ){

// }
// pub fn get_longest_path( 
//   models  : &'a Vec<Model>, 
//   nets    : &'a Vec<Net>, 
//   place   : &'a Vec<Placement> )
//   -> ( Vec<RoutingEvent>, u32 ) {

//   let mut rr_type_lists : Vec<(String,Vec<Route>)> = Vec::New();

//   for net_enum in nets {

//      //filter on net type
//     if let Local(ref net) = net_enum {

//       //get underlying routing tree
//       let routes : &Vec<Route> = net.route_tree.as_ref();



//       // if this route tree starts with a latched source or an input pad.
//       if models[0].latched.contains(&net.name) {

//         //starting at a latched output, if the route ends at a CLB sink (not PAD) 
//         //  then use the sink coords to find the ble name, then check if its latched or not.
//         // if latched end here.
//         // else travers that 

//       }
//     }
//     if
//   }
//   Ok(winner)
// }
















// pub fn routing_event_graph<'a>( models : &'a Vec<Model>, nets: &'a Vec<Net> ) 
//                               -> Result<HashMap<String,Vec<RoutingEvent>>> {

//   let mut rr_events = HashMap::<String,Vec<RoutingEvent>>::new();
//   // let mut g = GraphMap::new();
//   // let mut dag = Dag::<(RoutingEvent), u32, u32>::new(;

//   // // let mut dag = Dag::<Weight, u32, u32>::with_capacity(max_node, edges.len());


//   for net_enum in nets {

//     if let &Net::Local(ref net) = net_enum {

//       let rr : &Vec<Route> = net.route_tree.as_ref();

//       if models[0].latched.contains(&net.name) {
//         println!("Source found to be latched in {:} model", models[0].name);
//         // dag.add_node(Weight);


//       }
//     }

//     let max_node = 10;
//     let edges = &[(1, 4), (3, 4), (2, 5), (3, 5), (2, 8), (1, 9), (1, 8),
//                   (1, 3), (2, 7), (1, 7), (0, 6), (1, 2), (0, 7), (1, 6),
//                   (2, 4), (0, 1), (0, 9), (2, 9), (2, 6), (0, 4), (2, 3),
//                   (0, 2), (0, 3), (0, 5), (0, 8), (1, 5)];


//     for _ in 0..max_node {
//         dag.add_node(Weight);
//     }

//     // bench.iter(|| {
//         edges.iter().all(|&(a, b)|
//             dag.add_edge(NodeIndex::new(a), NodeIndex::new(b), 0).is_ok())
//     // });


//   }
//   Ok(rr_event_graph)
// }