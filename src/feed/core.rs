use std::collections::HashMap;

pub fn default_name() -> String {
    "generic".to_string()
}

pub trait BasicStream: Iterator {
    fn rename(&mut self, name: String);
    fn name(&self) -> String;
}

pub struct Constant<S: Clone> {
    value: S,
    name: String,
}

impl<S: Clone> Constant<S> {
    pub fn new(value: S) -> Constant<S> {
        Constant {
            name: default_name(),
            value: value,
        }
    }
}

impl<S: Clone> Iterator for Constant<S> {
    type Item = S;

    fn next(&mut self) -> Option<S> {
        Some(self.value.clone())
    }
}

impl<S: Clone> BasicStream for Constant<S> {
    fn rename(&mut self, name: String) {
        self.name = name;
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Stream<'a, S>
where
    S: Clone + 'a,
{
    it: Box<dyn Iterator<Item = S> + 'a>,
    name: String,
}

impl<'a, S: Clone> Stream<'a, S> {
    pub fn new<T>(it: T) -> Stream<'a, S>
    where
        T: Iterator<Item = S> + 'a,
    {
        Stream {
            name: default_name(),
            it: Box::new(it),
        }
    }

    pub fn source<T>(it: T) -> Stream<'a, S>
    where
        T: Iterator<Item = S> + 'a,
    {
        Stream {
            name: default_name(),
            it: Box::new(it),
        }
    }
}

impl<'a, S: Clone> Iterator for Stream<'a, S> {
    type Item = S;

    fn next(&mut self) -> Option<S> {
        self.it.next()
    }
}

impl<'a, S: Clone> BasicStream for Stream<'a, S> {
    fn rename(&mut self, name: String) {
        self.name = name;
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Group<'a, S>
where
    S: Clone,
{
    streams: HashMap<String, Box<dyn BasicStream<Item = S> + 'a>>,
    name: String,
}

impl<'a, S: Clone> Group<'a, S> {
    pub fn new<T>(streams: Vec<T>) -> Group<'a, S>
    where
        T: BasicStream<Item = S> + 'a,
    {
        let mut result: HashMap<String, Box<dyn BasicStream<Item = S>>> = HashMap::new();

        for stream in streams {
            result.insert(stream.name(), Box::new(stream));
        }

        Group {
            name: default_name(),
            streams: result,
        }
    }
}

impl<'a, S: Clone> Iterator for Group<'a, S> {
    type Item = HashMap<String, Option<S>>;

    fn next(&mut self) -> Option<HashMap<String, Option<S>>> {
        let mut result = HashMap::new();
        let mut is_empty = true;

        for (key, stream) in &mut self.streams {
            let value = stream.next().clone();

            if value.is_some() {
                is_empty = false;
            }

            result.insert(key.clone(), value);
        }

        match is_empty {
            true => None,
            false => Some(result),
        }
    }
}

impl<'a, S: Clone> BasicStream for Group<'a, S> {
    fn rename(&mut self, name: String) {
        self.name = name;
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}
