(function(){
  const TYPE_UNKNOWN = 1;
  const TYPE_LABEL = 2;
  const TYPE_BUTTON = 3;
  const TYPE_SELECT = 4;
  const TYPE_BIND = 5;
  const TYPE_INPUT = 6;
  const TYPE_HEADING = 7;
  const TYPE_HR = 8;
  const TYPE_BUTTON_UNKNOWN = 10;
  const TYPE_BUTTON_A = 11;
  const TYPE_BUTTON_BUTTON = 12;
  const TYPE_INPUT_RADIO = 20;
  const TYPE_INPUT_CHECKBOX = 21;
  const TYPE_INPUT_TEXTAREA = 22;
  const TYPE_INPUT_TEXT = 23;

  function TxtofPage(name) {
    this.name = name;
    this.rows = [new TxtofRow()];
    this.addRow = function(row) {
      this.rows.push(row);
    }
    this.getCurrentRow = function() {
      return this.rows[this.rows.length-1];
    }
  }
  function TxtofRow() { 
    this.cols = [new TxtofCol()]; 
    this.getCol = function(i) {
      return this.cols[i];
    }
    this.addCol = function(col) {
      this.cols.push(col);
    }
    this.colCount = function() {
      return this.cols.length;
    }    
  }
  function TxtofCol() {  
    this.buffers = [];
    this.size = '';
    this.addBuf = function() {
      this.buffers.push("");
    }
    this.setSize = function(c) {
      this.size = c;
    }
    this.append = function(c) {
      var len = this.buffers.length;
      if (len == 0) {
        this.addBuf();
        len = 1;
      }
      this.buffers[len-1] += c;
    }
    this.appendStr = function(s) {
      for (var i=0;i<s.length;i++) {
        this.append(s[i]);
      }
    }      
  }
  function TxtofWorking(char, type, secondaryType) {
    this.str = '';
    this.until = char;
    this.type = type;
    this.secondaryType = secondaryType;
    this.append = function(char) {
      this.str += char;
    }
    this.compile = function() {
      var template = this.secondaryType || this.type; 

      if (typeof(Txtof.templates[template]) == 'undefined') {
        return '';
      }

      template = Txtof.templates[template];

      var context = {};
      switch (true) {
        // Buttons
        case this.type == TYPE_BUTTON:
          var s = this.str.split("->");
          context = {
            value: s[0].trim(),
            trigger: s.length > 1 ? s[1].trim() : ''
          };
          break;
        // Inputs
        case this.type == TYPE_INPUT:
          var s = this.str.split("?");
          var value = s[0].trim();
          var placeholder = s.length > 1 ? s[1].trim() : "";
          s = this.str.split("->");
          context = {
            name: s.length > 1 ? s[1].trim() : '',
            value: value,
            placeholder: placeholder
          };
          break;
        // Select
        case this.type == TYPE_SELECT:
          context = {values: this.str.split(',')};
          break;
        // Everything else
        default:
          context = {value: this.str};
          break;
      }

      if (!template) {
        return "";
      }
      return template(context);
    }
  }
  var Txtof = {
    page: null,
    pages: [],
    templates: {},
    mode: null,
    working: null,
    colIndex: 1,
    init: function() {
      this.page = new TxtofPage("default");
      this.templates[TYPE_LABEL] =  Handlebars.compile($("#label-template").html());
      this.templates[TYPE_INPUT_TEXT] =  Handlebars.compile($("#text-template").html());
      this.templates[TYPE_INPUT_CHECKBOX] =  Handlebars.compile($("#checkbox-template").html());
      this.templates[TYPE_INPUT_RADIO] =  Handlebars.compile($("#radio-template").html());
      this.templates[TYPE_INPUT_TEXTAREA] =  Handlebars.compile($("#textarea-template").html());
      this.templates[TYPE_BUTTON_A] =  Handlebars.compile($("#a-template").html());
      this.templates[TYPE_BUTTON_BUTTON] =  Handlebars.compile($("#button-template").html());
      this.templates[TYPE_SELECT] =  Handlebars.compile($("#select-template").html());
      this.templates[TYPE_BIND] =  Handlebars.compile($("#bind-template").html());
      this.templates[TYPE_HEADING] =  Handlebars.compile($("#heading-template").html());
      this.templates[TYPE_HR] =  Handlebars.compile($("#hr-template").html());
    },
    parse: function(content) {
      this.init();
      var lines = content.split("\n");
      for (var i in lines) {
        Txtof.page.addRow(new TxtofRow());
        if (lines[i].trim() == "") {
          continue;
        }
        Txtof.working = null;
        Txtof.mode = null;
        Txtof.colIndex = 1;
        Txtof.parseChars(lines[i]);
      }
      Txtof.pages.push(Txtof.page);
      return Txtof.render();
    },
    parseChars: function(line) {
      for (var i = 0, len = line.length; i < len; i++) {
        var c = line[i];

        // Check leading characters
        if (i == 0) {
          switch (c) {
            case '=': return;
            case '-': 
              Txtof.page.getCurrentRow().getCol(Txtof.colIndex-1)
                .appendStr(new TxtofWorking('\0', TYPE_HR).compile());
              return;
            case '!': 
              Txtof.working = new TxtofWorking('\n', TYPE_HEADING);
              continue;
            case '|':
              Txtof.mode = 'form';
              continue;
            case '#':
              Txtof.pages.push(Txtof.page);
              Txtof.page = new TxtofPage(line.substring(1));
              return;
          } 
        }

        // Working mode
        if (Txtof.mode == 'form') {
          switch (true) {
            case /\d/.test(c) && line[i-1] == '|':
              Txtof.page.getCurrentRow().getCol(Txtof.colIndex-1)
                .setSize(c);
              continue;
            case c == '|':
              var row = Txtof.page.getCurrentRow();   
              if (row.colCount() < Txtof.colIndex + 1) {
                row.addCol(new TxtofCol());
              }
              Txtof.colIndex++;
              row.getCol(Txtof.colIndex-1).addBuf(); 
              continue ;
            case c == '[':
              Txtof.working = new TxtofWorking(']', TYPE_UNKNOWN);
              continue;
            case c == '{':
              Txtof.working = new TxtofWorking('}', TYPE_LABEL);
              continue;
            case c == '(':
              Txtof.working = new TxtofWorking(')', TYPE_BUTTON, TYPE_BUTTON_UNKNOWN);
              continue;
            case c == '<':
              Txtof.working = new TxtofWorking('>', TYPE_SELECT);
              continue;
            case c == '%':
              Txtof.working = new TxtofWorking(' ', TYPE_BIND);
              continue;
            case Txtof.working != null && c == Txtof.working.until:
              Txtof.page.getCurrentRow().getCol(Txtof.colIndex-1)
                .appendStr(Txtof.working.compile());
              Txtof.working = null;
              continue; 
            case Txtof.working != null && Txtof.working.type == TYPE_BUTTON && Txtof.working.secondaryType == TYPE_BUTTON_UNKNOWN:
              switch (true) {
                case c == '#':
                  Txtof.working.secondaryType = TYPE_BUTTON_A;
                  continue;
                default:
                  Txtof.working.secondaryType = TYPE_BUTTON_BUTTON;
                  Txtof.working.append(c);
                  continue;
              }
            case Txtof.working != null && Txtof.working.type == TYPE_UNKNOWN:
              Txtof.working.type = TYPE_INPUT;
              switch (true) {
                case c == 'o': 
                  Txtof.working.secondaryType = TYPE_INPUT_RADIO;
                  continue;
                case c == '/': 
                  Txtof.working.secondaryType = TYPE_INPUT_CHECKBOX;
                  continue;
                case c == '+': 
                  Txtof.working.secondaryType = TYPE_INPUT_TEXTAREA; 
                  continue;
                default:
                  Txtof.working.secondaryType = TYPE_INPUT_TEXT;
                  Txtof.working.append(c);
                  continue;
              }
          }
        }

        // Everything else is building strips
        if (Txtof.working != null) {
          Txtof.working.append(c);
        } else {
          Txtof.page.getCurrentRow().getCol(Txtof.colIndex-1).append(c);
        }    
      }

      if (Txtof.working) {
        Txtof.page.getCurrentRow().getCol(Txtof.colIndex-1)
                .appendStr(Txtof.working.compile());
      }
    },
    render: function() {
      var source   = $("#txtof-template").html();
      var template = Handlebars.compile(source);
      var context = {pages: Txtof.pages};
      $('#output').html(template(context));
      localStorage.removeItem('txtof');
      window.location.hash = 'default';
    }
  }

  // Handling navigation
  $(function(){             
    $(window).on('hashchange', function(e) {
      var data = JSON.parse(localStorage.getItem('txtof')) || {};
      for (var key in data) {
        $('[name=\"'+key+'\"]').val(data[key]);
        $('[data-bind=\"'+key+'\"]').html(data[key]);
      }
    });

    $(document).on('click', '[data-target]', function(){
      var data = JSON.parse(localStorage.getItem('txtof')) || {};
      data = $(this).closest('[txtof-container]').find(':input').serializeArray().reduce(function(m,o){m[o.name] = o.value; return m;}, data);
      localStorage.setItem('txtof', JSON.stringify(data));
      window.location.hash = $(this).data('target');
    });
  });
})();
