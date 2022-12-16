use std::collections::{HashMap, HashSet};
use nom::{
  IResult,
  bytes::complete::{tag, take_until},
  combinator:: opt,
  multi::many_till,
  character::complete::{alphanumeric1, line_ending, multispace0, digit1, space1},
  number::complete::float,
  sequence::{delimited, pair, tuple, terminated},}; 

#[derive(Debug, Clone)]
pub struct Mesh { 
    pub positions: Vec::<f32>,
    pub uv: Vec::<f32>,
    pub normals: Vec::<f32>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Blocks<'a> {
    header: (),
    nodes: (),
    skeleton: (),
    triangles: Vec<Triangle<'a>>,
    vertexanimation: (),
}

#[derive(Debug)]
pub struct Vertex {
    pub parent_bone : u32,
    pub pos_x : f32,
    pub pos_y : f32,
    pub pos_z : f32,
    pub norm_x : f32,
    pub norm_y : f32,
    pub norm_z : f32,
    pub u : f32,
    pub v : f32,
    pub links : u32,
    pub bone_id : u32,
    pub weight : f32,
}

#[derive(Debug)]
pub struct Triangle<'a> {
    pub texture : &'a str,
    pub v1 : Vertex,
    pub v2 : Vertex,
    pub v3 : Vertex,
}

fn parse_vertex(input: &str) -> IResult<&str, Vertex> {
    let (rest, 
            (d, _, 
             (pos_x, _, 
             pos_y, _, 
             pos_z, _, 
             ),
             (norm_x, _, 
             norm_y, _, 
             norm_z, _,
             ),
             (u, _, 
             v, _,
             ),
             (links, _, 
             bone_id, _,
             weight,
             ),
             )) = tuple((digit1, space1,
                           tuple((
                               float, space1,
                               float, space1, 
                               float, space1,
                           )),
                           tuple((
                               float, space1, 
                               float, space1, 
                               float, space1, 
                           )),
                           tuple((
                               float, space1, 
                               float, space1,
                           )),
                           tuple((
                               digit1, space1, 
                               digit1, space1,
                               float, 
                           )),
                         ))(input)?;
    let vertex = 
            Vertex {
                parent_bone: d.parse().map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))?,
                pos_x: pos_x,
                pos_y: pos_y,
                pos_z: pos_z,
                norm_x: norm_x,
                norm_y: norm_y,
                norm_z: norm_z,
                u: u,
                v: v,
                links: links.parse().map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))?,
                bone_id: bone_id.parse().map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))?,
                weight: weight,
        };
    Ok((rest, vertex))
}


fn parse_triangle(input: &str) -> IResult<&str, Triangle> {
    let (rest, ( tex, (v1,v2,v3))) = 
        pair(terminated(alphanumeric1, line_ending),
             tuple(
                (delimited(multispace0, parse_vertex, line_ending),
                 delimited(multispace0, parse_vertex, line_ending),
                 delimited(multispace0, parse_vertex, line_ending),
                )),
            )
            (input)?;

    Ok( ( rest,
        Triangle {
            texture: tex, 
            v1: v1,
            v2: v2,
            v3: v3,
        },
        )
    )
}


fn parse_triangles(input: &str) ->  IResult<&str, Vec<Triangle>> {
    let (rest, (_, triangles)) =
            pair( pair(tag("triangles"), line_ending)
                , many_till(parse_triangle, pair(tag("end"), line_ending))
                )
                (input)?;
    Ok((rest, triangles.0))
}


fn parse_skeleton(input: &str) ->  IResult<&str, ()> {
    let (rest, (_, _, _)) =
            tuple((pair(tag("skeleton"), line_ending),
                  take_until("end"),
                  pair(tag("end"), line_ending),
                  ))
                (input)?;
    Ok((rest, ()))
}

fn parse_nodes(input: &str) ->  IResult<&str, ()> {
    let (rest, (_, _, _)) =
            tuple((pair(tag("nodes"), line_ending),
                  take_until("end"),
                  pair(tag("end"), line_ending),
                  ))
                (input)?;
    Ok((rest, ()))
}

fn parse_header(input: &str) ->  IResult<&str, ()> {
    let (rest, (_, _, _, _,)) =
            tuple((tag("version"),
                   multispace0,
                   digit1,
                   line_ending,
                  ))(input)?;
    Ok((rest, ()))
}

fn parse_vertexanimation(input: &str) ->  IResult<&str, ()> {
    let (rest, (_, _, _,)) =
            tuple((pair(tag("vertexanimation"), line_ending),
                  take_until("end"),
                  pair(tag("end"), line_ending),
                  ))
                (input)?;
    Ok((rest, ()))
}

fn parse_blocks(input: &str) ->  IResult<&str, Blocks> {
    let (rest, (header, nodes, skeleton, triangles, _)) =  
            tuple((parse_header, 
                   parse_nodes, 
                   parse_skeleton, 
                   parse_triangles,
                   opt(parse_vertexanimation),
                   ))(input)?;

    Ok((rest, Blocks {
        skeleton: skeleton,
        triangles: triangles,
        header: header,
        nodes: nodes,
        vertexanimation: (),
    }))
}


pub fn parse_smd(file: &str) -> Result<Vec<(String, Mesh)>, String> {
    match parse_blocks(&file) {
        Ok((_,res)) => {

            let mut textures = HashSet::new();
            for t in &res.triangles {
                textures.insert(t.texture);
            }

            let mut positions : HashMap<&str, (Vec<f32>,  Vec<f32>, Vec<f32>)> = HashMap::new();

            for t in &textures {
                positions.insert(t, (Vec::new(), Vec::new(), Vec::new()));
            }
            for t in res.triangles {
                positions.get_mut(t.texture).unwrap().0.extend_from_slice(&vec!
                       [t.v1.pos_x, t.v1.pos_y, t.v1.pos_z, 
                        t.v2.pos_x, t.v2.pos_y, t.v2.pos_z,
                        t.v3.pos_x, t.v3.pos_y, t.v3.pos_z
                       ]
                   );
                positions.get_mut(t.texture).unwrap().1.extend_from_slice(&vec!
                       [t.v1.norm_x, t.v1.norm_y, t.v1.norm_z, 
                        t.v2.norm_x, t.v2.norm_y, t.v2.norm_z,
                        t.v3.norm_x, t.v3.norm_y, t.v3.norm_z
                       ]
                   );
                positions.get_mut(t.texture).unwrap().2.extend_from_slice(&vec!
                       [t.v1.u, t.v1.v,  
                        t.v2.u, t.v2.v, 
                        t.v3.u, t.v3.v, 
                       ]
                   );
            }

            Ok(positions.iter().map(|(key, val)|
                                 (String::from(key.clone()),
                                  Mesh {
                                      positions: val.0.clone(),
                                      normals: val.1.clone(),
                                      uv: val.2.clone(),
                                  },
                                 )).collect::<Vec<(String, Mesh)>>()
            )
        },
        Err(e) => Err(format!("Parsing error: {}", e)),
    }
}
