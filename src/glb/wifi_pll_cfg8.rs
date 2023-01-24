#[doc = "Register `wifi_pll_cfg8` reader"]
pub struct R(crate::R<WIFI_PLL_CFG8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg8` writer"]
pub struct W(crate::W<WIFI_PLL_CFG8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WIFI_PLL_CFG8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_en_div2` reader - "]
pub type WIFIPLL_EN_DIV2_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div2` writer - "]
pub type WIFIPLL_EN_DIV2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_en_div4` reader - "]
pub type WIFIPLL_EN_DIV4_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div4` writer - "]
pub type WIFIPLL_EN_DIV4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_en_div5` reader - "]
pub type WIFIPLL_EN_DIV5_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div5` writer - "]
pub type WIFIPLL_EN_DIV5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_en_div6` reader - "]
pub type WIFIPLL_EN_DIV6_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div6` writer - "]
pub type WIFIPLL_EN_DIV6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_en_div8` reader - "]
pub type WIFIPLL_EN_DIV8_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div8` writer - "]
pub type WIFIPLL_EN_DIV8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_en_div10` reader - "]
pub type WIFIPLL_EN_DIV10_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div10` writer - "]
pub type WIFIPLL_EN_DIV10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_en_div12` reader - "]
pub type WIFIPLL_EN_DIV12_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div12` writer - "]
pub type WIFIPLL_EN_DIV12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_en_div20` reader - "]
pub type WIFIPLL_EN_DIV20_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div20` writer - "]
pub type WIFIPLL_EN_DIV20_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_en_div30` reader - "]
pub type WIFIPLL_EN_DIV30_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_en_div30` writer - "]
pub type WIFIPLL_EN_DIV30_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
#[doc = "Field `wifipll_sel_div2_div4` reader - "]
pub type WIFIPLL_SEL_DIV2_DIV4_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_sel_div2_div4` writer - "]
pub type WIFIPLL_SEL_DIV2_DIV4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG8_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_en_div2(&self) -> WIFIPLL_EN_DIV2_R {
        WIFIPLL_EN_DIV2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifipll_en_div4(&self) -> WIFIPLL_EN_DIV4_R {
        WIFIPLL_EN_DIV4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifipll_en_div5(&self) -> WIFIPLL_EN_DIV5_R {
        WIFIPLL_EN_DIV5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_en_div6(&self) -> WIFIPLL_EN_DIV6_R {
        WIFIPLL_EN_DIV6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifipll_en_div8(&self) -> WIFIPLL_EN_DIV8_R {
        WIFIPLL_EN_DIV8_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wifipll_en_div10(&self) -> WIFIPLL_EN_DIV10_R {
        WIFIPLL_EN_DIV10_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wifipll_en_div12(&self) -> WIFIPLL_EN_DIV12_R {
        WIFIPLL_EN_DIV12_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wifipll_en_div20(&self) -> WIFIPLL_EN_DIV20_R {
        WIFIPLL_EN_DIV20_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_en_div30(&self) -> WIFIPLL_EN_DIV30_R {
        WIFIPLL_EN_DIV30_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wifipll_sel_div2_div4(&self) -> WIFIPLL_SEL_DIV2_DIV4_R {
        WIFIPLL_SEL_DIV2_DIV4_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div2(&mut self) -> WIFIPLL_EN_DIV2_W<0> {
        WIFIPLL_EN_DIV2_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div4(&mut self) -> WIFIPLL_EN_DIV4_W<1> {
        WIFIPLL_EN_DIV4_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div5(&mut self) -> WIFIPLL_EN_DIV5_W<2> {
        WIFIPLL_EN_DIV5_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div6(&mut self) -> WIFIPLL_EN_DIV6_W<3> {
        WIFIPLL_EN_DIV6_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div8(&mut self) -> WIFIPLL_EN_DIV8_W<4> {
        WIFIPLL_EN_DIV8_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div10(&mut self) -> WIFIPLL_EN_DIV10_W<5> {
        WIFIPLL_EN_DIV10_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div12(&mut self) -> WIFIPLL_EN_DIV12_W<6> {
        WIFIPLL_EN_DIV12_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div20(&mut self) -> WIFIPLL_EN_DIV20_W<7> {
        WIFIPLL_EN_DIV20_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div30(&mut self) -> WIFIPLL_EN_DIV30_W<8> {
        WIFIPLL_EN_DIV30_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sel_div2_div4(&mut self) -> WIFIPLL_SEL_DIV2_DIV4_W<9> {
        WIFIPLL_SEL_DIV2_DIV4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg8](index.html) module"]
pub struct WIFI_PLL_CFG8_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg8::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg8::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg8 to value 0x01ff"]
impl crate::Resettable for WIFI_PLL_CFG8_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff;
}
