const assetsSRC     = 'assets/src/',
      assetsDST     = 'assets/build/',
      components    = 'components/';

const gulp          = require('gulp'),
      sass          = require('gulp-sass')(require('sass')),
      sourcemaps    = require('gulp-sourcemaps'),
      fs            = require('fs'),
      path          = require('path'),
      merge         = require('merge-stream'),
      postcss       = require('gulp-postcss'),
      autoprefixer  = require('gulp-autoprefixer'),
      plumber       = require('gulp-plumber'),
      concat        = require('gulp-concat'),
      uglify        = require('gulp-uglify'),
      rename        = require('gulp-rename'),
      strip         = require('gulp-strip-comments');

const componentCSS    = components + '**/*.scss',
      componentJS     = components + '**/*.js',
      globalCSS       = assetsSRC + 'scss/style.scss';
      configCSS       = assetsSRC + 'scss/config/**.scss';
      themeCSS        = assetsSRC + 'scss/theme/**.scss';
      globalJS        = assetsSRC + 'js/*.js';
      libJS           = assetsSRC + 'js/lib/*.js';

var sassOptions = {
  errLogToConsole: true,
  outputStyle: 'compressed'
};

function getFolders(dir) {
  return fs.readdirSync(dir).filter(function(file) {
    return fs.statSync(path.join(dir, file)).isDirectory();
  });
}


// function css() {
//   var folders = getFolders(components);
//   var tasks = folders.map(function(folder) {
//     var src = path.join(components, folder);
//     var dst = path.join(assetsDST + 'css');
//     console.log(src);
//     console.log(path.join(src, '**/*.scss'));
// 
//     // return gulp.src(path.join(src, '/**/*.scss'))
//     return gulp.src(path.join(src, '**/*.scss'))
//       // .pipe(sourcemaps.init())
//       .pipe(sass(sassOptions).on('error', sass.logError))
//       .pipe(autoprefixer('last 2 versions'))
//       // .pipe(sourcemaps.write('.'))
//       // .pipe(concat("style.min.css"))
//       .pipe(gulp.dest(dst));
//   });
// 
//   let m = merge(tasks);
//   // console.log(m);
//   return m;
// }


function css() {
  var folders = getFolders(components);
  var tasks = folders.map(function(folder) {
    var src = path.join(components, folder);
    var dst = path.join(assetsDST + 'css/');
    console.log(src, dst);
    var file_path = path.join(src, '**/component.scss');
    console.log(file_path);
    return gulp.src(file_path)
      // .pipe(sourcemaps.init())
      .pipe(sass(sassOptions).on('error', sass.logError))
      .pipe(autoprefixer('last 2 versions'))
      .pipe(rename(function(file) {
        file.dirname = path.dirname(file.dirname);
        file.basename = file.basename;
        return file;
      }))
      // .pipe(sourcemaps.write('.'))
      .pipe(concat("style.min.css"))
      .pipe(gulp.dest(dst));
  });
  return merge(tasks);
}


function mainCSS() {
  var dst = path.join(assetsDST + 'css/');
  return gulp.src(globalCSS, { allowEmpty: true })
  .pipe(sourcemaps.init())
  .pipe(sass(sassOptions).on('error', sass.logError))
  .pipe(autoprefixer('last 2 versions'))
  .pipe(sourcemaps.write('.'))
  .pipe(gulp.dest(dst));
}


function libraryJS() {
  var dst = path.join(assetsDST + 'JS/lib');
  return gulp.src(libJS).pipe(gulp.dest(dst));
}


function watch() {
  gulp.watch([globalCSS,configCSS,themeCSS, componentCSS],mainCSS);
}


exports.mainCSS = mainCSS;
// exports.mainJS = mainJS;
exports.libraryJS = libraryJS;
exports.css = css;
// exports.javascript = javascript;
exports.watch = watch;

const build = gulp.series(watch);
gulp.task('default', build);
